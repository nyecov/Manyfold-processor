import logging
import os
from handlers.base_handler import BaseHandler

logger = logging.getLogger("orphan_cleanup_plugin")

class OrphanCleanupPlugin(BaseHandler):
    def process(self) -> bool:
        logger.info(f"Processing Orphan/Generic: {self.filename}")
        
        try:
            # For orphans, we just move the file to its destination folder.
            # BaseHandler.move_to_output already creates the folder based on the slug.
            # No Datapackage JSON.
            
            self.move_to_output([self.input_path])
            
            # The file is moved, so no separate cleanup needed if move_to_output used shutil.move
            return True

        except Exception as e:
            logger.error(f"Failed to process Orphan: {e}")
            return False
