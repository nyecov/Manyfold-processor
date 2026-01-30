import struct
import zipfile
import io
import logging

logger = logging.getLogger("legacy_converters")

class LegacyConverters:
    @staticmethod
    def stl_to_3mf_pure_python(filepath: str, target_path: str):
        """
        [ARCHIVED 2026-01-26] 
        Lightweight STL to 3MF conversion using only standard libraries.
        Replaced by Rust optimizer (stl23mf) for better performance.
        Kept for historical reference.
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
                    return False 

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
            logger.warning(f"Legacy Pure Python conversion failed: {e}")
            return False
