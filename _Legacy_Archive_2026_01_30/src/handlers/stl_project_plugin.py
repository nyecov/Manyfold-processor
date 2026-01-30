import logging
import os
import shutil
import tempfile
from handlers.base_handler import BaseHandler
from utils.metadata import DatapackageGenerator
from utils.image_converter import ImageConverter
from utils.geometry_converter import GeometryConverter

logger = logging.getLogger("stl_project_plugin")

class StlProjectPlugin(BaseHandler):
    def process(self) -> bool:
        logger.info(f"Processing STL Project for: {self.filename}")
        if len(self.input_paths) > 1:
            logger.info(f"Aggregating {len(self.input_paths)} STLs.")

        try:
            # 1. Metadata (Filename derived)
            # If multiple files, use the first one's base name or the folder name if we had it.
            # BaseHandler already set self.slug and self.filename based on input_paths[0]
            title = self.filename.replace(".stl", "").replace("-", " ").title()
            if len(self.input_paths) > 1:
                title += " (Aggregated)"
            
            # 2. Identify Siblings (Images) for all STLs
            # We unique them to avoid duplicates if multiple STLs share images
            siblings = set()
            preview_image = None
            
            # Helper for tokenization
            import re
            def tokenize(s):
                return set(re.split(r'[-_\s\.]+', s.lower()))

            for stl_path in self.input_paths:
                stl_base = os.path.splitext(os.path.basename(stl_path))[0]
                search_dirs = [os.path.dirname(stl_path), self.staging_dir]
                
                for dir_path in search_dirs:
                    if not os.path.exists(dir_path): continue
                    
                    try:
                        for f in os.listdir(dir_path):
                            f_path = os.path.join(dir_path, f)
                            # Skip if this file is one of our inputs
                            if f_path in self.input_paths: continue
                            
                            f_base = os.path.splitext(f)[0]
                            is_image = f.lower().endswith(('.jpg', '.jpeg', '.png', '.webp'))
                            if not is_image: continue

                            tokens_stl = tokenize(stl_base)
                            tokens_sibling = tokenize(f_base)
                            common = tokens_stl.intersection(tokens_sibling)
                            
                            match = False
                            for t in common:
                                if len(t) > 3: 
                                    match = True
                                    break
                            
                            if match or stl_base in f_base or f_base in stl_base:
                                logger.info(f"Found sibling for {stl_base}: {f}")
                                siblings.add(f_path)
                                if not preview_image:
                                    preview_image = f

                    except Exception as e:
                        logger.warning(f"Error searching for siblings in {dir_path}: {e}")

            # Fallback: Single Image Rule (Implicit Association)
            # Only if we found zero siblings for ALL STLs and there is exactly one image in the primary dir
            if not siblings and self.input_paths:
                try:
                    primary_dir = os.path.dirname(self.input_path)
                    all_images = []
                    for f in os.listdir(primary_dir):
                        if any(f == os.path.basename(p) for p in self.input_paths): continue
                        if f.lower().endswith(('.jpg', '.jpeg', '.png', '.webp')):
                            all_images.append(os.path.join(primary_dir, f))
                    
                    if len(all_images) == 1:
                        logger.info(f"Single Image Rule applied to project")
                        siblings.add(all_images[0])
                        preview_image = os.path.basename(all_images[0])
                        
                except Exception as e:
                    logger.warning(f"Error in Single Image Rule: {e}")
            
            # --- PHASE 2b: CONVERSION ---
            with tempfile.TemporaryDirectory() as temp_work_dir:
                final_artifacts = []
                
                # 3a. Convert STL(s) -> 3MF
                converted_3mf = None
                if len(self.input_paths) == 1:
                    converted_3mf = GeometryConverter.stl_to_3mf(self.input_path, temp_work_dir, os.path.splitext(self.filename)[0])
                else:
                    converted_3mf = GeometryConverter.merge_stls_to_3mf(self.input_paths, temp_work_dir, os.path.splitext(self.filename)[0])
                
                model_resource_name = ""
                model_type = ""

                if converted_3mf:
                    final_artifacts.append(converted_3mf)
                    model_resource_name = os.path.basename(converted_3mf)
                    model_type = "model/3mf"
                else:
                    # Fallback to original (if single) or just fail if merging failed?
                    # For now, fallback to first one if single, or handle fail.
                    if len(self.input_paths) == 1:
                        final_artifacts.append(self.input_path)
                        model_resource_name = self.filename
                        model_type = "model/stl"
                    else:
                        logger.error("Aggregation failed and no fallback for multi-file.")
                        return False

                # 3b. Convert Images -> WebP
                converted_images = []
                processed_siblings = []

                for sib in list(siblings):
                     sib_base = os.path.basename(sib)
                     webp_path = ImageConverter.to_webp(sib, temp_work_dir, sib_base)
                     if webp_path:
                         converted_images.append(webp_path)
                         processed_siblings.append(sib)
                     else:
                         converted_images.append(sib)
                
                final_artifacts.extend(converted_images)

                # 4. Generate Datapackage
                dp = DatapackageGenerator(self.slug, title)
                dp.add_resource(model_resource_name, model_resource_name, model_type)
                
                for img_path in converted_images:
                    img_name = os.path.basename(img_path)
                    mtype = "image/webp" if img_name.endswith(".webp") else "image/jpeg"
                    dp.add_resource(os.path.splitext(img_name)[0], img_name, mtype)

                json_path = os.path.join(temp_work_dir, "datapackage.json")
                dp.write(json_path) 
                
                if not os.path.exists(json_path) and os.path.exists("datapackage.json"):
                    shutil.move("datapackage.json", json_path)
                
                final_artifacts.append(json_path)
                
                # 5. Move Artifacts
                self.move_to_output(final_artifacts)
            
            # 6. Cleanup Originals
            try:
                # If we converted to 3MF, delete all original STLs
                if converted_3mf:
                    for stl_path in self.input_paths:
                        if os.path.exists(stl_path):
                            os.remove(stl_path)
                            logger.info(f"Deleted original STL: {stl_path}")
                
                for sib in processed_siblings:
                    if os.path.exists(sib):
                        os.remove(sib)
                        logger.info(f"Deleted original Image: {sib}")
                        
            except Exception as e:
                logger.warning(f"Error cleaning up original files: {e}")

            return True

        except Exception as e:
            logger.error(f"Failed to process STL Project: {e}")
            import traceback
            traceback.print_exc()
            return False
