import logging
import os
import subprocess
import time
import struct
import zipfile
import io

logger = logging.getLogger("geometry_converter")

# Path to the Rust optimizer binary
# Path to the Rust optimizer binary
RUST_OPTIMIZER = "/app/plugins/stl23mf/stl23mf"
if not os.path.exists(RUST_OPTIMIZER):
    # Fallback for local development/testing outside container
    RUST_OPTIMIZER = os.path.join(os.path.dirname(__file__), "..", "plugins", "stl23mf", "target", "release", "stl23mf")

class GeometryConverter:
    @staticmethod
    def _stl_to_3mf_pure_python(filepath: str, target_path: str):
        """
        Lightweight STL to 3MF conversion using only standard libraries.
        Supports Binary STL. ASCII STL will fallback.
        """
        try:
            with open(filepath, 'rb') as f:
                header = f.read(80)
                count_bytes = f.read(4)
                if len(count_bytes) < 4: return False
                count = struct.unpack('<I', count_bytes)[0]
                
                # Check if it's likely ASCII (heuristic)
                f.seek(0)
                start = f.read(100).lower()
                if b'solid' in start and b'facet' in start:
                    return False # ASCII STL not supported in lightweight pure-python yet

                f.seek(84)
                
                # Generate 3MF XML
                model_xml = io.StringIO()
                model_xml.write('<?xml version="1.0" encoding="UTF-8"?>\n')
                model_xml.write('<model unit="millimeter" xml:lang="en-US" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02">\n')
                model_xml.write('  <resources>\n')
                model_xml.write('    <object id="1" type="model">\n')
                model_xml.write('      <mesh>\n')
                model_xml.write('        <vertices>\n')
                
                vertices = []
                vertex_map = {} # (x,y,z) -> index
                triangles = []
                
                for _ in range(count):
                    data = f.read(50)
                    if len(data) < 50: break
                    # Normal(12), V1(12), V2(12), V3(12), Attr(2)
                    v_data = struct.unpack('<fff fff fff fff H', data)
                    
                    for i in range(3, 12, 3):
                        v = (v_data[i], v_data[i+1], v_data[i+2])
                        if v not in vertex_map:
                            idx = len(vertices)
                            vertex_map[v] = idx
                            vertices.append(v)
                            model_xml.write(f'          <vertex x="{v[0]}" y="{v[1]}" z="{v[2]}" />\n')
                    
                    triangles.append((vertex_map[(v_data[3], v_data[4], v_data[5])],
                                      vertex_map[(v_data[6], v_data[7], v_data[8])],
                                      vertex_map[(v_data[9], v_data[10], v_data[11])]))
                
                model_xml.write('        </vertices>\n')
                model_xml.write('        <triangles>\n')
                for t in triangles:
                    model_xml.write(f'          <triangle v1="{t[0]}" v2="{t[1]}" v3="{t[2]}" />\n')
                model_xml.write('        </triangles>\n')
                model_xml.write('      </mesh>\n')
                model_xml.write('    </object>\n')
                model_xml.write('  </resources>\n')
                model_xml.write('  <build>\n')
                model_xml.write('    <item objectid="1" />\n')
                model_xml.write('  </build>\n')
                model_xml.write('</model>\n')
                
                # Create ZIP
                with zipfile.ZipFile(target_path, 'w', zipfile.ZIP_DEFLATED) as z:
                    z.writestr("[Content_Types].xml", '<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/></Types>')
                    z.writestr("_rels/.rels", '<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Target="/3D/3dmodel.model" Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/></Relationships>')
                    z.writestr("3D/3dmodel.model", model_xml.getvalue())
                
                return True
        except Exception as e:
            logger.warning(f"Pure Python conversion failed: {e}")
            return False

    @staticmethod
    def stl_to_3mf(filepath: str, output_dir: str, base_name: str) -> str:
        """
        Converts an STL file to 3MF format.
        Returns the absolute path of the generated 3MF file.
        Attempts: Rust -> Pure Python -> Trimesh
        """
        target_path = os.path.join(output_dir, f"{base_name}.3mf")
        
        # 1. Try Rust Optimizer
        if os.path.exists(RUST_OPTIMIZER):
            try:
                start_time = time.time()
                logger.info(f"Using Rust optimizer for {filepath}")
                
                # Use Popen to stream updates
                process = subprocess.Popen(
                    [RUST_OPTIMIZER, "--input", filepath, "--output", target_path],
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    bufsize=1
                )
                
                # Stream output
                while True:
                    line = process.stdout.readline()
                    if not line and process.poll() is not None:
                        break
                    if line:
                        line = line.strip()
                        if line.startswith("PROGRESS:"):
                            logger.info(line)
                        elif line.startswith("Error:"):
                            logger.error(line)
                        # else ignore noisy debug output if any
                        
                rc = process.poll()

                if rc == 0:
                    duration = time.time() - start_time
                    logger.info(f"Rust conversion successful in {duration:.2f}s")
                    return target_path
                else:
                    stderr = process.stderr.read()
                    logger.warning(f"Rust optimizer failed (RC={rc}): {stderr}")
            except Exception as e:
                logger.warning(f"Error calling Rust optimizer: {e}")
        
        # 2. Fallback to Trimesh (Trusted Backup)
        try:
            import trimesh
            logger.info("Falling back to Trimesh...")
            start_time = time.time()
            mesh = trimesh.load(filepath, file_type='stl')
            mesh.export(target_path, file_type='3mf')
            duration = time.time() - start_time
            logger.info(f"Trimesh conversion successful in {duration:.2f}s")
            return target_path
            
        except Exception as e:
            logger.error(f"Trimesh fallback failed: {e}")
            return None
    @staticmethod
    def merge_stls_to_3mf(filepaths: list, output_dir: str, base_name: str) -> str:
        """
        Aggregates multiple STL files into a single 3MF.
        Returns the absolute path of the generated 3MF file.
        Preferentially uses Rust optimizer for 3MF generation to keep objects separate.
        Falls back to Trimesh if Rust tool is missing.
        """
        target_path = os.path.join(output_dir, f"{base_name}.3mf")
        
        # 1. Try Rust Optimizer (Preferred)
        if os.path.exists(RUST_OPTIMIZER):
            try:
                start_time = time.time()
                logger.info(f"Using Rust optimizer for merging {len(filepaths)} files")
                
                # Construct command: stl23mf --input file1 --input file2 ... --output target
                cmd = [RUST_OPTIMIZER]
                for fp in filepaths:
                    cmd.extend(["--input", fp])
                cmd.extend(["--output", target_path])

                # Use Popen to stream updates
                process = subprocess.Popen(
                    cmd,
                    stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE,
                    text=True,
                    bufsize=1
                )
                
                # Stream output
                while True:
                    line = process.stdout.readline()
                    if not line and process.poll() is not None:
                        break
                    if line:
                        line = line.strip()
                        if line.startswith("PROGRESS:"):
                            logger.info(line)
                        elif line.startswith("Error:"):
                            logger.error(line)
                        # else ignore noisy debug output if any
                        
                rc = process.poll()

                if rc == 0:
                    duration = time.time() - start_time
                    logger.info(f"Rust merge successful in {duration:.2f}s")
                    return target_path
                else:
                    stderr = process.stderr.read()
                    logger.warning(f"Rust optimizer merge failed (RC={rc}): {stderr}")
            except Exception as e:
                logger.warning(f"Error calling Rust optimizer for merge: {e}")
        
        # 2. Fallback to Trimesh (Only if Rust missing or failed)
        try:
            import trimesh
            logger.warning(f"Rust optimizer unavailable or failed. Falling back to Trimesh for {target_path}")
            start_time = time.time()
            
            meshes = []
            for fp in filepaths:
                m = trimesh.load(fp)
                # If it's a scene, get all geometries
                if isinstance(m, trimesh.Scene):
                    for g in m.geometry.values():
                        meshes.append(g)
                else:
                    meshes.append(m)
            
            # Use Scene export which handles multiple objects better for 3MF
            scene = trimesh.Scene(meshes)
            scene.export(target_path, file_type='3mf')
            
            duration = time.time() - start_time
            logger.info(f"Trimesh scene export successful in {duration:.2f}s")
            return target_path

        except Exception as e:
            logger.error(f"Trimesh fallback failed: {e}")
            return None
