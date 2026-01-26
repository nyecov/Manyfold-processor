import logging
import os
import shutil
import tempfile
from handlers.base_handler import BaseHandler
from utils.metadata import DatapackageGenerator
from utils.image_converter import ImageConverter
from utils.geometry_converter import GeometryConverter

logger = logging.getLogger("case1_handler")

class Case1STLHandler(BaseHandler):
    def process(self) -> bool:
        logger.info(f"Processing Case 1 (STL) for: {self.filename}")
        
        # Assumption: Manager has already checked 'Staging Lock'. 
        # We are free to move files.
        try:
            # 1. Metadata (Filename derived)
            title = self.filename.replace(".stl", "").replace("-", " ").title()
            
            # 2. Identify Siblings (Images)
            base_name = os.path.splitext(self.filename)[0]
            # Search locations: Input dir (current) AND Staging dir (if it got moved early)
            search_dirs = [os.path.dirname(self.input_path), self.staging_dir]
            
            siblings = []
            preview_image = None
            
            for dir_path in search_dirs:
                if not os.path.exists(dir_path): continue
                
                try:
                    for f in os.listdir(dir_path):
                        f_path = os.path.join(dir_path, f)
                        if f == self.filename and dir_path == os.path.dirname(self.input_path): 
                            continue # Skip self if in input
                        
                        f_base = os.path.splitext(f)[0]
                        # Fix: Define is_image before usage
                        is_image = f.lower().endswith(('.jpg', '.jpeg', '.png', '.webp'))

                        # Improved Heuristic: Token Intersection
                        import re
                        def tokenize(s):
                            return set(re.split(r'[-_\s\.]+', s.lower()))
                        
                        tokens_stl = tokenize(base_name)
                        tokens_sibling = tokenize(f_base)
                        
                        common = tokens_stl.intersection(tokens_sibling)
                        
                        # Debug Log
                        logger.info(f"Checking {f}: STL={tokens_stl} SIB={tokens_sibling} COMMON={common}")

                        match = False
                        for t in common:
                            if len(t) > 3: 
                                match = True
                                break
                        
                        if is_image and (match or base_name in f_base or f_base in base_name):
                            logger.info(f"Found sibling: {f} (Match={match})")
                            siblings.append(f_path)
                            if not preview_image:
                                preview_image = f

                except Exception as e:
                    logger.warning(f"Error searching for siblings in {dir_path}: {e}")

            # Fallback: Single Image Rule (Implicit Association)
            if not siblings:
                try:
                    primary_dir = os.path.dirname(self.input_path)
                    all_images = []
                    for f in os.listdir(primary_dir):
                        if f == self.filename: continue
                        if f.lower().endswith(('.jpg', '.jpeg', '.png', '.webp')):
                            all_images.append(os.path.join(primary_dir, f))
                    
                    if len(all_images) == 1:
                        logger.info(f"Single Image Rule: Implicitly grouping {all_images[0]} with {self.filename}")
                        siblings.append(all_images[0])
                        preview_image = os.path.basename(all_images[0])
                        
                except Exception as e:
                    logger.warning(f"Error in Single Image Rule: {e}")
            
            # --- PHASE 2b: CONVERSION ---
            # Use a temp directory for conversion work
            with tempfile.TemporaryDirectory() as temp_work_dir:
                
                final_artifacts = []
                
                # 3a. Convert STL -> 3MF
                converted_3mf = GeometryConverter.stl_to_3mf(self.input_path, temp_work_dir, base_name)
                
                model_resource_name = ""
                model_type = ""

                if converted_3mf:
                    final_artifacts.append(converted_3mf)
                    model_resource_name = os.path.basename(converted_3mf)
                    model_type = "model/3mf"
                else:
                    # Fallback to original
                    final_artifacts.append(self.input_path)
                    model_resource_name = self.filename
                    model_type = "model/stl"

                # 3b. Convert Images -> WebP
                converted_images = []
                # Keep track of which siblings were successfully converted to delete originals later
                processed_siblings = []

                for sib in siblings:
                     sib_base = os.path.basename(sib)
                     webp_path = ImageConverter.to_webp(sib, temp_work_dir, sib_base)
                     if webp_path:
                         converted_images.append(webp_path)
                         processed_siblings.append(sib)
                     else:
                         converted_images.append(sib) # Fallback to original path
                
                final_artifacts.extend(converted_images)

                # 4. Generate Datapackage
                dp = DatapackageGenerator(self.slug, title)
                dp.add_resource(model_resource_name, model_resource_name, model_type)
                
                for img_path in converted_images:
                    img_name = os.path.basename(img_path)
                    # Helper to guess mime type if fallback
                    mtype = "image/webp" if img_name.endswith(".webp") else "image/jpeg"
                    dp.add_resource(os.path.splitext(img_name)[0], img_name, mtype)

                # Write JSON to temp dir
                json_path = os.path.join(temp_work_dir, "datapackage.json")
                dp.write(json_path) 
                
                # Check if it wrote to CWD (default) or Path. 
                # DatapackageGenerator.write takes 'filename'. If it's just 'datapackage.json', it writes to CWD.
                # Let's move it if it landed in CWD.
                # Assuming simple implementation in metadata.py: 
                # with open(filename, 'w') as f: json.dump(...)
                # If generated via path, it's fine.
                
                if not os.path.exists(json_path) and os.path.exists("datapackage.json"):
                    shutil.move("datapackage.json", json_path)
                
                final_artifacts.append(json_path)
                
                # 5. Move Artifacts (Converted + Config) to Output
                self.move_to_output(final_artifacts)
            
            # 6. Cleanup Originals
            # Only delete if we successfully converted (i.e. we didn't use the original pathway)
            try:
                if converted_3mf and os.path.exists(self.input_path):
                    os.remove(self.input_path)
                    logger.info(f"Deleted original STL: {self.input_path}")
                
                for sib in processed_siblings:
                    if os.path.exists(sib):
                        os.remove(sib)
                        logger.info(f"Deleted original Image: {sib}")
                        
            except Exception as e:
                logger.warning(f"Error cleaning up original files: {e}")

            return True

        except Exception as e:
            logger.error(f"Failed to process Case 1: {e}")
            import traceback
            traceback.print_exc()
            return False
