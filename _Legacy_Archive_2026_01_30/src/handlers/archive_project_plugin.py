import logging
import os
import shutil
import tempfile
import patoolib
from handlers.base_handler import BaseHandler
from handlers.directory_project_plugin import DirectoryProjectPlugin

logger = logging.getLogger("archive_project_plugin")

class ArchiveProjectPlugin(BaseHandler):
    def process(self) -> bool:
        logger.info(f"Processing Archive: {self.filename}")
        
        # We extract to a temporary directory
        # Then we treat the contents of that directory as a 'Directory Project'
        try:
            with tempfile.TemporaryDirectory() as extract_dir:
                logger.info(f"Extracting {self.filename} to {extract_dir}")
                
                # patoolib handles zip, rar, 7z etc.
                patoolib.extract_archive(self.input_path, outdir=extract_dir, interactive=False)
                
                # Now we have the content.
                # Is it a single folder inside? Or multiple files?
                # Manyfold usually likes a folder.
                # We'll use DirectoryProjectPlugin's logic.
                
                # We need to give it a "fake" input path that is the extract_dir
                # But we want the slug to be derived from the ARCHIVE name, not the temp dir name.
                
                # We can create a sub-handler or just use the logic.
                # Using the plugin directly:
                dir_plugin = DirectoryProjectPlugin(extract_dir, self.output_dir, self.staging_dir)
                # Override the slug to match the archive name
                dir_plugin.slug = self.slug
                
                success = dir_plugin.process()
                
                if success:
                    # Cleanup the archive itself
                    if os.path.exists(self.input_path):
                        os.remove(self.input_path)
                    return True
                else:
                    return False

        except Exception as e:
            logger.error(f"Failed to process Archive: {e}")
            import traceback
            traceback.print_exc()
            return False
