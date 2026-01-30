import zipfile
import os
import xml.dom.minidom

def inspect(path):
    print(f"--- Inspecting {os.path.basename(path)} ---")
    if not os.path.exists(path):
        print(f"File not found: {path}")
        return

    try:
        with zipfile.ZipFile(path, 'r') as z:
            print("Files in zip:")
            for n in z.namelist():
                print(f"  {n}")
            
            print("\nContent of 3D/3dmodel.model (truncated):")
            try:
                with z.open("3D/3dmodel.model") as f:
                    content = f.read().decode('utf-8')
                    # Pretty print xml
                    dom = xml.dom.minidom.parseString(content)
                    print(dom.toprettyxml(indent="  ")[:2000]) # First 2000 chars
            except KeyError:
                print("  3D/3dmodel.model not found")

            print("\nContent of Metadata/model_settings.config (truncated):")
            try:
                with z.open("Metadata/model_settings.config") as f:
                    content = f.read().decode('utf-8')
                     # Pretty print xml
                    dom = xml.dom.minidom.parseString(content)
                    print(dom.toprettyxml(indent="  ")[:2000])
            except KeyError:
                print("  Metadata/model_settings.config not found")
    except Exception as e:
        print(f"Error reading zip: {e}")
    print("\n" + "="*50 + "\n")

ref_path = r"c:\Users\Furiosa\Manyfold Processor\Manyfold-processor\test_data\Loose Files STL_and_jpg\Dragon+2+Big+and+Beautiful.3mf"
gen_path = r"c:\Users\Furiosa\Manyfold Processor\Manyfold-processor\playground\output\pauldron-plates\Pauldron_plates.3mf"

inspect(ref_path)
inspect(gen_path)
