import zipfile
import logging
import os
import xml.etree.ElementTree as ET
from handlers.base_handler import BaseHandler
from utils.metadata import DatapackageGenerator

logger = logging.getLogger("case2_handler")

class Case2MakerWorldHandler(BaseHandler):
    def process(self) -> bool:
        logger.info(f"Processing Case 2 (MakerWorld) for: {self.filename}")
        
        try:
            with zipfile.ZipFile(self.input_path, 'r') as z:
                # 1. Parse Metadata from XML
                # Note: MakerWorld often uses 'Metadata/project_settings.config' or 3dmodel.model
                # We'll stick to the standard 3dmodel.model first
                
                title = self.filename # Fallback
                description = ""
                
                if '3D/3dmodel.model' in z.namelist():
                     with z.open('3D/3dmodel.model') as f:
                        tree = ET.parse(f)
                        root = tree.getroot()
                        
                        # Find all metadata entries (looking for Title/Description)
                        # We use a simple tag match ignoring namespaces
                        for elem in root.iter():
                             if elem.tag.split('}')[-1] == 'metadata':
                                  name = elem.get('name')
                                  if name == 'Title':
                                      title = elem.text or title
                                  elif name == 'Description':
                                      description = elem.text or description

                # 2. Extract Thumbnail
                # Look for typical paths
                thumb_path = None
                possible_thumbs = [
                    "Metadata/pick_1.png", 
                    "Metadata/top_1.png", 
                    "Metadata/Thumbnail.png",
                    "Auxiliaries/.thumbnails/thumbnail_middle.png" 
                ]
                
                extracted_thumb = None
                for t in possible_thumbs:
                    if t in z.namelist():
                        extracted_thumb = os.path.basename(t)
                        with open(extracted_thumb, 'wb') as f:
                            f.write(z.read(t))
                        break
                
                # 3. Generate Datapackage
                dp = DatapackageGenerator(self.slug, title)
                dp.set_description(description)
                
                # Add the 3MF itself (we just copy it later, but list it here)
                dp.add_resource(self.slug, self.filename, "model/3mf")
                
                artifacts = []
                
                if extracted_thumb:
                    # In a real app we'd convert to webp here
                    dp.add_resource(self.slug + "_preview", extracted_thumb, "image/png")
                    artifacts.append(extracted_thumb)
                
                # Write JSON
                json_name = "datapackage.json"
                dp.write(json_name)
                artifacts.append(json_name)
                
                # Copy original 3MF to CWD/Artifacts list (to be moved)
                # In this logic, we are moving the *Input* file to output
                # So we add the input path to the list of things to move? 
                # Actually BaseHandler.move_to_output takes paths. 
                # We usually want to move the INPUT file too.
                artifacts.append(self.input_path) 
                
                self.move_to_output(artifacts)
                return True

        except Exception as e:
            logger.error(f"Failed to process Case 2: {e}")
            return False
