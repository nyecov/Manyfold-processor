import logging
import os
import shutil
import zipfile
from handlers.case2_makerworld import Case2MakerWorldHandler
from handlers.case3_generic import Case3GenericHandler
from handlers.case1_stl import Case1STLHandler

logger = logging.getLogger("processor_manager")

class ProcessorManager:
    def __init__(self):
        # Use env vars or defaults
        self.staging_dir = os.getenv("STAGING_DIR", "/staging")
        self.output_dir = os.getenv("OUTPUT_DIR", "/output")
        self.input_dir = os.getenv("INPUT_DIR", "/input")
        self.auto_off_when_done = os.getenv("AUTO_OFF_WHEN_DONE", "false").lower() == "true"
        self.config_file = "/config/config.json"
        
        # Load Defaults
        self.config = {
            "auto_process": True,
            "theme": "light"
        }
        self.state = "waiting"
        self.errors = [] # List of {"timestamp": ..., "error": ..., "file": ...}
        self._load_config()

        # [SAFETY] Always start with auto_process OFF regardless of config
        self.config["auto_process"] = False
        logger.info("System initialized with Auto-Process [OFF] for safety.")

    @property
    def auto_process(self):
        return self.config.get("auto_process", True)

    @auto_process.setter
    def auto_process(self, value):
        logger.info(f"SETTING AUTO-PROCESS: {value} (Type: {type(value)})")
        self.config["auto_process"] = value
        self._save_config()
        if value is True:
            logger.info("Auto-process enabled. Scanning queues...")
            self._scan_staging()
            self._scan_input()

    @property
    def theme(self):
        return self.config.get("theme", "light")

    @theme.setter
    def theme(self, value):
        self.config["theme"] = value
        self._save_config()

    def _load_config(self):
        try:
            if os.path.exists(self.config_file):
                with open(self.config_file, 'r') as f:
                    import json
                    loaded = json.load(f)
                    self.config.update(loaded)
                    logger.info(f"Loaded config: {self.config}")
        except Exception as e:
            logger.error(f"Failed to load config: {e}")

    def _save_config(self):
        try:
            with open(self.config_file, 'w') as f:
                import json
                json.dump(self.config, f, indent=2)
                logger.info("Config saved.")
        except Exception as e:
            logger.error(f"Failed to save config: {e}")


    def handle_event(self, filepath):
        """Called when a new file is detected"""
        logger.info(f"Handling event for: {filepath}")
        
        # 1. Auto-Process Check (Priority 1)
        if not self.auto_process:
            logger.info("Auto-process is OFF. Moving to Staging...")
            self._move_to_staging(filepath)
            return

        # 2. Safety Lock
        is_model = filepath.lower().endswith(('.stl', '.3mf', '.zip', '.obj'))
        if not self._is_staging_empty() and not is_model:
            self.state = "locked"
            logger.warning(f"STAGING LOCK: Staging is not empty and file is not a model.")
            return
        
        self.state = "processing"
        handler = self._identify_handler(filepath)
        
        if handler:
            success = handler.process()
            if not success:
               self._record_error(f"Processing failed for {os.path.basename(filepath)}", filepath)
               logger.warning(f"Processing failed for {filepath}, moving to staging.")
               self._move_to_staging(filepath)
            else:
               if self._is_staging_empty():
                   self._scan_input()
        else:
            self._record_error(f"No handler identified for {os.path.basename(filepath)}", filepath)
            logger.warning(f"No handler identified for {filepath}, moving to staging.")
            self._move_to_staging(filepath)
        
        self.state = "waiting"
        self._check_auto_reset()

    def _check_auto_reset(self):
        """Resets auto_process to False if both Input and Staging are empty"""
        if self.auto_process and self._is_input_empty() and self._is_staging_empty():
            logger.info("Auto-Reset triggered: All queues empty. Disabling Auto-Process.")
            self.config["auto_process"] = False
            self._save_config()

    def _scan_input(self):
        """Scans input directory for stranded files and triggers event handling"""
        if not os.path.exists(self.input_dir):
            return
            
        logger.info(f"Scanning Input for stranded files: {self.input_dir}")
        try:
            files = [f for f in os.listdir(self.input_dir) if not f.startswith('.')]
            if not files:
                logger.info("Input is empty.")
                return

            for filename in files:
                filepath = os.path.join(self.input_dir, filename)
                # We simply call handle_event again. 
                # If Staging is truly empty, it will accept them now.
                # Note: This might recurse if we process immediately?
                # handle_event calls process calls scan_input... 
                # Recursion depth?
                # If Auto-Process is ON, handle_event processes synchronously.
                # So we recurse. 
                # This is actually fine for a queue, basically consuming the input queue.
                # But to avoid stack overflow on massive queues, maybe we should cycle?
                # For now, 2 files scenario -> fine.
                logger.info(f"Retrying stranded file: {filename}")
                self.handle_event(filepath)
                    
        except Exception as e:
            logger.error(f"Error scanning input: {e}")

    def _identify_handler(self, filepath):
        """Detects the appropriate handler based on file content"""
        try:
            if zipfile.is_zipfile(filepath):
                 return Case2MakerWorldHandler(filepath, self.output_dir, self.staging_dir)
            
            # Not a zip. Is it an STL?
            if filepath.lower().endswith('.stl'):
                return Case1STLHandler(filepath, self.output_dir, self.staging_dir)
            
            # Fallback: Generic/Image
            # If it's an image or known type that fell through (orphan), use Case 3.
            if filepath.lower().endswith(('.jpg', '.png', '.jpeg', '.webp')):
                 return Case3GenericHandler(filepath, self.output_dir, self.staging_dir)
                 
        except Exception:
            # Fallback
            pass
            
        return None

    def _is_staging_empty(self):
        """Returns True if staging directory has no files (ignoring dotfiles)"""
        try:
            if not os.path.exists(self.staging_dir):
                return True
            files = [f for f in os.listdir(self.staging_dir) if not f.startswith('.')]
            return len(files) == 0
        except Exception:
            return False

    def _is_input_empty(self):
        """Returns True if input directory has no files (ignoring dotfiles)"""
        try:
            if not os.path.exists(self.input_dir):
                return True
            files = [f for f in os.listdir(self.input_dir) if not f.startswith('.')]
            return len(files) == 0
        except Exception:
            return False

    def _move_to_staging(self, filepath):
        target = os.path.join(self.staging_dir, os.path.basename(filepath))
        shutil.move(filepath, target)
        logger.info(f"Moved {filepath} to {target}")

    def process_file(self, filename):
        """Manually process a specific file from Staging"""
        filepath = os.path.join(self.staging_dir, filename)
        if not os.path.exists(filepath):
            self._record_error(f"File not found in staging: {filename}", filepath)
            logger.error(f"File not found in staging: {filename}")
            return False
            
        self.state = "processing"
        handler = self._identify_handler(filepath)
        if handler:
            success = handler.process()
            self.state = "waiting"
            return success
        else:
             self._record_error(f"No handler for {filename}", filepath)
             logger.warning(f"No handler for {filename}")
             self.state = "waiting"
             return False

    def _record_error(self, message, filepath=None):
        import time
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        error_entry = {
            "timestamp": timestamp,
            "error": message,
            "file": os.path.basename(filepath) if filepath else "N/A"
        }
        self.errors.append(error_entry)
        # Keep only last 50 errors
        if len(self.errors) > 50:
            self.errors.pop(0)

    def process_all_loose(self):
        """Process all staged files manually"""
        self._scan_staging()

    def _scan_staging(self):
        """Scans staging directory for processable models and triggers them"""
        if not os.path.exists(self.staging_dir):
            return
            
        logger.info(f"Scanning Staging: {self.staging_dir}")
        processed_count = 0
        try:
            files = [f for f in os.listdir(self.staging_dir) if not f.startswith('.')]
            # Sort models first
            def sort_key(f):
                ext = os.path.splitext(f)[1].lower()
                return 0 if ext in ['.stl', '.3mf', '.zip'] else 1
            files.sort(key=sort_key)
            
            for filename in files:
                filepath = os.path.join(self.staging_dir, filename)
                handler = self._identify_handler(filepath)
                
                if handler:
                   logger.info(f"Processable item found in Staging: {filename}")
                   success = handler.process()
                   if success:
                       processed_count += 1
                   else:
                       logger.warning(f"Failed to process staged item: {filename}")
                else:
                    pass
            
            # Auto-Off Logic: Only reset if we actually did work
            if processed_count > 0:
                self._check_auto_reset()

        except Exception as e:
            logger.error(f"Error scanning staging: {e}")
