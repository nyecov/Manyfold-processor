import logging
import os
import shutil
import tempfile
import xml.etree.ElementTree as ET
from handlers.base_handler import BaseHandler
from utils.metadata import DatapackageGenerator
from utils.image_converter import ImageConverter

logger = logging.getLogger("native_3mf_plugin")

class Native3mfPlugin(BaseHandler):
    def process(self) -> bool:
        logger.info(f"Processing Native 3MF for: {self.filename}")
        
        try:
            # 1. Metadata (Filename derived)
            title = self.filename.replace(".3mf", "").replace("-", " ").title()
            
            # 2. Extract Metadata/Thumbnails if possible
            # 3MF is a ZIP.
            import zipfile
            
            description = ""
            thumbnails = []
            
            with zipfile.ZipFile(self.input_path, 'r') as z:
                # Try to find metadata in 3D/3dmodel.model
                if '3D/3dmodel.model' in z.namelist():
                    try:
                        with z.open('3D/3dmodel.model') as f:
                            tree = ET.parse(f)
                            root = tree.getroot()
                            for elem in root.iter():
                                if elem.tag.split('}')[-1] == 'metadata':
                                    name = elem.get('name')
                                    if name == 'Title':
                                        title = elem.text or title
                                    elif name == 'Description':
                                        description = elem.text or description
                    except Exception as e:
                        logger.warning(f"Could not parse metadata from 3MF: {e}")

                # Try to find thumbnails in Metadata/
                for name in z.namelist():
                    if name.lower().startswith('metadata/') and name.lower().endswith(('.png', '.jpg', '.jpeg')):
                        thumbnails.append(name)

            # 3. Handle Artifacts in Temp
            with tempfile.TemporaryDirectory() as temp_work_dir:
                final_artifacts = []
                
                # Copy original 3MF
                # Actually, we can just move the original directly later, but let's be consistent.
                primary_3mf = os.path.join(temp_work_dir, self.filename)
                shutil.copy2(self.input_path, primary_3mf)
                final_artifacts.append(primary_3mf)
                
                processed_images = []
                # Extract and convert thumbnails
                with zipfile.ZipFile(self.input_path, 'r') as z:
                    for i, thumb in enumerate(thumbnails):
                        ext = os.path.splitext(thumb)[1]
                        thumb_temp = os.path.join(temp_work_dir, f"thumb_{i}{ext}")
                        with open(thumb_temp, 'wb') as f:
                            f.write(z.read(thumb))
                        
                        webp_path = ImageConverter.to_webp(thumb_temp, temp_work_dir, f"preview_{i}")
                        if webp_path:
                            processed_images.append(webp_path)
                        else:
                            # Fallback
                            processed_images.append(thumb_temp)

                final_artifacts.extend(processed_images)
                
                # 4. Generate Datapackage
                dp = DatapackageGenerator(self.slug, title)
                dp.set_description(description)
                dp.add_resource(self.slug, self.filename, "model/3mf")
                
                for img in processed_images:
                    img_name = os.path.basename(img)
                    dp.add_resource(os.path.splitext(img_name)[0], img_name, "image/webp")
                
                json_path = os.path.join(temp_work_dir, "datapackage.json")
                dp.write(json_path)
                final_artifacts.append(json_path)
                
                # 5. Move to Output
                self.move_to_output(final_artifacts)

            # 6. Cleanup Original
            if os.path.exists(self.input_path):
                os.remove(self.input_path)
            
            return True

        except Exception as e:
            logger.error(f"Failed to process Native 3MF: {e}")
            return False
