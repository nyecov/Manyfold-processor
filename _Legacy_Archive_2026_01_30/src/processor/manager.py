import logging
import os
import shutil
import zipfile
from handlers.stl_project_plugin import StlProjectPlugin
from handlers.native_3mf_plugin import Native3mfPlugin
from handlers.archive_project_plugin import ArchiveProjectPlugin
from handlers.directory_project_plugin import DirectoryProjectPlugin
from handlers.orphan_cleanup_plugin import OrphanCleanupPlugin

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
        if not os.path.exists(filepath):
            logger.info(f"File vanished before processing: {filepath}")
            return
            
        # Safety: NEVER process the root input or staging dir
        abs_path = os.path.abspath(filepath)
        input_root = os.path.abspath(self.input_dir)
        staging_root = os.path.abspath(self.staging_dir)
        
        if abs_path == input_root or abs_path == staging_root:
            return

        # NEW SAFETY: Ignore nested events. We only process TOP-LEVEL items in /input and /staging.
        # This prevents picking up subfolders (like images/ or models/) as separate projects.
        parent_dir = os.path.abspath(os.path.dirname(filepath))
        if parent_dir != input_root and parent_dir != staging_root:
            logger.debug(f"Ignoring nested event: {filepath}")
            return

        logger.info(f"Handling event for: {filepath}")
        
        # 1. Auto-Process Check (Priority 1)
        if not self.auto_process:
            logger.info("Auto-process is OFF. Moving to Staging...")
            self._move_to_staging(filepath)
            return

        # 2. Safety Lock
        is_model = os.path.isdir(filepath) or filepath.lower().endswith(('.stl', '.3mf', '.zip', '.rar', '.7z', '.obj'))
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
            if os.path.exists(filepath):
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
        """Detects the appropriate handler based on file/folder content"""
        # Normalize paths for comparison
        filepath = os.path.abspath(filepath)
        input_root = os.path.abspath(self.input_dir)
        staging_root = os.path.abspath(self.staging_dir)

        try:
            # Case 4: Project Folder
            if os.path.isdir(filepath):
                 # Safety: NEVER treat the root input or staging dir as a project folder
                 if filepath == input_root or filepath == staging_root:
                     logger.debug(f"Ignoring root directory: {filepath}")
                     return None
                 return DirectoryProjectPlugin(filepath, self.output_dir, self.staging_dir)

            ext = filepath.lower().rsplit('.', 1)[-1] if '.' in filepath else ''

            # Case 3: Archive (.zip, .rar, .7z)
            if ext in ['zip', 'rar', '7z']:
                 return ArchiveProjectPlugin(filepath, self.output_dir, self.staging_dir)

            # Case 1: STL (Aggregation check)
            if ext == 'stl':
                # Check for other STLs in the same directory for aggregation
                parent = os.path.dirname(filepath)
                stls = []
                for f in os.listdir(parent):
                    if f.lower().endswith('.stl'):
                        stls.append(os.path.join(parent, f))
                
                # If multiple STLs, we pass all of them
                return StlProjectPlugin(stls, self.output_dir, self.staging_dir)

            # Case 2: Native 3MF
            if ext == '3mf':
                return Native3mfPlugin(filepath, self.output_dir, self.staging_dir)

            # Case 5: Generic/Orphan (Images, etc.)
            if ext in ['jpg', 'png', 'jpeg', 'webp', 'obj']:
                 return OrphanCleanupPlugin(filepath, self.output_dir, self.staging_dir)

        except Exception as e:
            logger.error(f"Error identifying handler: {e}")
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
