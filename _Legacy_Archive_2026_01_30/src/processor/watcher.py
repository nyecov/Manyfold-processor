import time
import os
import threading
from watchdog.events import FileSystemEventHandler
from watchdog.observers import Observer
from watchdog.observers.polling import PollingObserver
import logging

logger = logging.getLogger("watchdog_service")

# Time to wait for no new events before processing (seconds)
SETTLE_DELAY = 5.0 

class DebounceHandler(FileSystemEventHandler):
    def __init__(self, service):
        self.service = service

    def _register(self, filepath):
        filename = os.path.basename(filepath)
        if filename.startswith('.'): return
        
        logger.info(f"Event detected: {filename} (Debouncing...)")
        self.service.register_event(filepath)

    def on_created(self, event):
        self._register(event.src_path)

    def on_moved(self, event):
        self._register(event.dest_path)
            
    def on_modified(self, event):
        # Optional: track modifications if files are written slowly
        self._register(event.src_path)

class WatchdogService:
    def __init__(self, manager, input_dir="/input"):
        self.manager = manager
        self.input_dir = input_dir
        
        # Use PollingObserver if requested (better for Docker on Windows/Network drives)
        if os.getenv("WATCHDOG_POLLING", "false").lower() == "true":
            logger.info("Using PollingObserver (CPU intensive but reliable for binds)")
            self.observer = PollingObserver()
        else:
            self.observer = Observer()
            
        self.handler = DebounceHandler(self)
        
        # Debounce state
        self.pending_files = {} # {filepath: last_seen_time}
        self.lock = threading.Lock()
        self.running = False
        self.worker_thread = None

    def register_event(self, filepath):
        with self.lock:
            self.pending_files[filepath] = time.time()

    def _process_loop(self):
        """Background thread to check for settled files"""
        while self.running:
            time.sleep(0.5)
            
            now = time.time()
            settled_files = []
            
            with self.lock:
                # Identify files that haven't changed for SETTLE_DELAY
                keys = list(self.pending_files.keys())
                for filepath in keys:
                    last_seen = self.pending_files[filepath]
                    if now - last_seen > SETTLE_DELAY:
                        settled_files.append(filepath)
                        del self.pending_files[filepath]
            
            # Process settled files outside the lock
            # Sort them: Folders and Models first
            def sort_key(f):
                if os.path.isdir(f): return 0
                ext = os.path.splitext(f)[1].lower()
                return 1 if ext in ['.stl', '.3mf', '.zip', '.rar', '.7z'] else 2
            
            settled_files.sort(key=sort_key)
            
            for filepath in settled_files:
                # Check if file still exists (wasn't deleted during debounce)
                if os.path.exists(filepath):
                    logger.info(f"File settled, processing: {os.path.basename(filepath)}")
                    try:
                        self.manager.handle_event(filepath)
                    except Exception as e:
                        logger.error(f"Error processing settled file {filepath}: {e}")

    def start(self):
        if not os.path.exists(self.input_dir):
            logger.warning(f"Input directory {self.input_dir} does not exist. Creating it.")
            os.makedirs(self.input_dir, exist_ok=True)

        # 2. Start Worker Thread
        self.running = True
        self.worker_thread = threading.Thread(target=self._process_loop, daemon=True)
        self.worker_thread.start()

        # 3. Start Startup Scan in Background
        startup_thread = threading.Thread(target=self._startup_scan, daemon=True)
        startup_thread.start()

    def _startup_scan(self):
        """Scans input directory for existing items (folders/files) and handles them"""
        logger.info("Performing startup scan of input directory...")
        try:
            # We only look at TOP LEVEL items in /input
            items = [os.path.join(self.input_dir, f) for f in os.listdir(self.input_dir) if not f.startswith('.')]
            
            def sort_key(f):
                if os.path.isdir(f): return 0
                ext = os.path.splitext(f)[1].lower()
                return 1 if ext in ['.stl', '.3mf', '.zip', '.rar', '.7z'] else 2
            
            items.sort(key=sort_key)
            
            for filepath in items:
                 logger.info(f"Startup scan found: {os.path.basename(filepath)}")
                 try:
                    self.manager.handle_event(filepath)
                 except Exception as e:
                    logger.error(f"Error processing startup item {filepath}: {e}")

        except Exception as e:
            logger.error(f"Error during startup scan: {e}")

        # 3. Start Real-time Monitoring
        self.observer.schedule(self.handler, self.input_dir, recursive=True)
        self.observer.start()
        logger.info(f"Watchdog observing: {self.input_dir} (Recursive=True, Debounce={SETTLE_DELAY}s)")
    
    def stop(self):
        self.running = False
        self.observer.stop()
        self.observer.join()
        if self.worker_thread:
            self.worker_thread.join(timeout=2.0)
