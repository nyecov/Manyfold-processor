import logging
import zipfile
from handlers.base_handler import BaseHandler
from utils.metadata import DatapackageGenerator

logger = logging.getLogger("case3_handler")

class Case3GenericHandler(BaseHandler):
    def process(self) -> bool:
        logger.info(f"Processing Case 3 (Generic) for: {self.filename}")
        
        try:
            # 1. Determine Type
            is_zip = zipfile.is_zipfile(self.input_path)
            
            # Basic Metadata (Filename derived)
            title = self.filename.replace(".3mf", "").replace(".zip", "").replace("-", " ").title()
            # Remove extension from title if present
            if "." in title:
                title = title.rsplit(".", 1)[0]
            
            # Determine if we should generate a JSON (only for 3MF/ZIP projects)
            # Loose images/orphans skip JSON as per user request
            should_generate_json = is_zip or self.filename.endswith(('.stl', '.3mf'))
            
            artifacts = [self.input_path]
            
            if should_generate_json:
                # Generate Datapackage
                dp = DatapackageGenerator(self.slug, title)
                
                resource_type = "model/3mf" if self.filename.endswith(".3mf") else "application/zip"
                if not is_zip:
                    # Infer type for loose files (though usually STL falls here if handled as Case 3)
                    ext = self.filename.split('.')[-1].lower()
                    if ext in ['jpg', 'jpeg', 'png', 'webp']:
                        resource_type = f"image/{ext}"
                    else:
                        resource_type = "application/octet-stream"
                
                dp.add_resource(self.slug, self.filename, resource_type)
                
                # Write JSON
                json_name = "datapackage.json"
                dp.write(json_name)
                artifacts.append(json_name)
            
            # Move
            self.move_to_output(artifacts)
            return True

        except Exception as e:
            logger.error(f"Failed to process Case 3: {e}")
            return False
