import logging
import os
from PIL import Image

logger = logging.getLogger("image_converter")

class ImageConverter:
    @staticmethod
    def to_webp(filepath: str, output_dir: str, base_name: str) -> str:
        """
        Converts an image to WebP format.
        Returns the absolute path of the generated WebP file.
        """
        try:
            filename = os.path.basename(filepath)
            name_no_ext = os.path.splitext(base_name)[0]
            target_path = os.path.join(output_dir, f"{name_no_ext}.webp")
            
            logger.info(f"Converting image {filename} to WebP...")
            
            with Image.open(filepath) as img:
                # Convert to RGB if RGBA/P to avoid JPEG/WebP issues (though WebP supports alpha)
                # WebP supports RGBA, so we usually don't need to force RGB unless saving as JPG.
                # But let's be safe for thumbnails.
                img.save(target_path, "WEBP", quality=80, method=4)
                
            logger.info(f"Generated WebP: {target_path}")
            return target_path
            
        except Exception as e:
            logger.error(f"Failed to convert image {filepath}: {e}")
            return None
