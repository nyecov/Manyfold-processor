import os
import shutil
import logging
from abc import ABC, abstractmethod

logger = logging.getLogger("base_handler")

class BaseHandler(ABC):
    def __init__(self, input_paths, output_dir: str, staging_dir: str):
        if isinstance(input_paths, str):
            self.input_paths = [input_paths]
        else:
            self.input_paths = input_paths

        self.input_path = self.input_paths[0] if self.input_paths else ""
        self.output_dir = output_dir
        self.staging_dir = staging_dir
        self.filename = os.path.basename(self.input_path) if self.input_path else "unknown"
        self.slug = self._sanitize_slug(os.path.splitext(self.filename)[0])

    def _sanitize_slug(self, text: str) -> str:
        """Sanitizes the filename to be a valid folder name/slug"""
        # Replace spaces, +, and underscores with dashes
        slug = text.lower().replace(" ", "-").replace("+", "-").replace("_", "-")
        # Remove any other non-alphanumeric (except dashes)
        return "".join(c for c in slug if c.isalnum() or c in "-")

    def _create_dedicated_folder(self, base_dir: str) -> str:
        """Creates the dedicated subfolder rule"""
        target_dir = os.path.join(base_dir, self.slug)
        os.makedirs(target_dir, exist_ok=True)
        return target_dir

    def move_to_output(self, artifacts: list):
        """Moves processed artifacts to the final dedicated output folder"""
        target_dir = self._create_dedicated_folder(self.output_dir)
        
        for artifact in artifacts:
            if os.path.exists(artifact):
                shutil.move(artifact, os.path.join(target_dir, os.path.basename(artifact)))
        
        logger.info(f" moved artifacts to {target_dir}")

    @abstractmethod
    def process(self) -> bool:
        """Main processing logic. Returns True if successful."""
        pass
