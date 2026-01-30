import logging
import os
import shutil
import tempfile
from handlers.base_handler import BaseHandler
from handlers.stl_project_plugin import StlProjectPlugin
from utils.metadata import DatapackageGenerator
from utils.image_converter import ImageConverter
from utils.geometry_converter import GeometryConverter

logger = logging.getLogger("directory_project_plugin")

class DirectoryProjectPlugin(BaseHandler):
    def __init__(self, input_path: str, output_dir: str, staging_dir: str):
        super().__init__(input_path, output_dir, staging_dir)
        # For directories, the input_path IS the directory
        self.directory = input_path
        # Re-sanitize slug to be the folder name
        self.slug = self._sanitize_slug(os.path.basename(self.directory))

    def process(self) -> bool:
        logger.info(f"Processing Directory Project: {self.directory}")
        
        try:
            # 1. Scan for Models and Images
            model_files = []
            image_files = []
            other_files = []
            
            for root, dirs, files in os.walk(self.directory):
                for f in files:
                    f_path = os.path.join(root, f)
                    ext = f.lower().split('.')[-1]
                    if ext in ['stl', '3mf', 'obj']:
                        model_files.append(f_path)
                    elif ext in ['jpg', 'jpeg', 'png', 'webp']:
                        image_files.append(f_path)
                    else:
                        other_files.append(f_path)

            if not model_files and not image_files:
                logger.warning(f"Directory {self.directory} has no models or images. Skipping.")
                return False

            title = os.path.basename(self.directory).replace("-", " ").replace("_", " ").title()

            with tempfile.TemporaryDirectory() as temp_work_dir:
                final_artifacts = []
                
                # 2. Process Models
                # If multiple models, aggregate them into one 3MF
                model_resource_name = ""
                model_type = ""
                
                if len(model_files) == 1:
                    ext = model_files[0].lower().split('.')[-1]
                    if ext == 'stl':
                        converted = GeometryConverter.stl_to_3mf(model_files[0], temp_work_dir, self.slug)
                        if converted:
                            final_artifacts.append(converted)
                            model_resource_name = os.path.basename(converted)
                            model_type = "model/3mf"
                        else:
                            # Fallback
                            shutil.copy2(model_files[0], os.path.join(temp_work_dir, os.path.basename(model_files[0])))
                            model_resource_name = os.path.basename(model_files[0])
                            model_type = "model/stl"
                    else:
                        # Already 3MF or other
                        target = os.path.join(temp_work_dir, os.path.basename(model_files[0]))
                        shutil.copy2(model_files[0], target)
                        final_artifacts.append(target)
                        model_resource_name = os.path.basename(target)
                        model_type = "model/3mf" if ext == "3mf" else "application/octet-stream"
                
                elif len(model_files) > 1:
                    # Aggregation
                    logger.info(f"Aggregating {len(model_files)} models in directory.")
                    # Note: merge_stls_to_3mf actually uses trimesh which supports both STL/3MF/OBJ!
                    # Let's use it for the general case.
                    aggregated = GeometryConverter.merge_stls_to_3mf(model_files, temp_work_dir, self.slug)
                    if aggregated:
                        final_artifacts.append(aggregated)
                        model_resource_name = os.path.basename(aggregated)
                        model_type = "model/3mf"
                    else:
                        # Fail aggregation? Or just copy all?
                        # For now, let's copy all as fallback
                        for mf in model_files:
                            target = os.path.join(temp_work_dir, os.path.basename(mf))
                            shutil.copy2(mf, target)
                            final_artifacts.append(target)
                        model_resource_name = os.path.basename(model_files[0])
                        model_type = "model/3mf" # (Estimated)

                # 3. Process Images
                converted_images = []
                for img in image_files:
                    webp = ImageConverter.to_webp(img, temp_work_dir, os.path.splitext(os.path.basename(img))[0])
                    if webp:
                        converted_images.append(webp)
                    else:
                        target = os.path.join(temp_work_dir, os.path.basename(img))
                        shutil.copy2(img, target)
                        converted_images.append(target)
                
                final_artifacts.extend(converted_images)

                # 4. Generate Datapackage
                dp = DatapackageGenerator(self.slug, title)
                if model_resource_name:
                    dp.add_resource(self.slug, model_resource_name, model_type)
                
                for img in converted_images:
                    img_name = os.path.basename(img)
                    dp.add_resource(os.path.splitext(img_name)[0], img_name, "image/webp")
                
                json_path = os.path.join(temp_work_dir, "datapackage.json")
                dp.write(json_path)
                final_artifacts.append(json_path)
                
                # 5. Move to Output
                self.move_to_output(final_artifacts)

            # 6. Cleanup Original Directory
            if os.path.exists(self.directory):
                shutil.rmtree(self.directory)
            
            return True

        except Exception as e:
            logger.error(f"Failed to process Directory: {e}")
            import traceback
            traceback.print_exc()
            return False
