use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};

// --- Minimal ZIP Implementation (Store Only) ---
// 3MF allow uncompressed (Store) method. This avoids needing `flate2` dependency.

struct ZipEntry {
    name: String,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    offset: u32,
}

struct ZipWriter<W: Write + Seek> {
    writer: W,
    entries: Vec<ZipEntry>,
}

impl<W: Write + Seek> ZipWriter<W> {
    fn new(writer: W) -> Self {
        Self { writer, entries: Vec::new() }
    }

    fn add_file(&mut self, name: &str, data: &[u8]) -> io::Result<()> {
        let offset = self.writer.stream_position()? as u32;
        let crc = crc32_simple(data);
        let size = data.len() as u32;

        // Local File Header
        self.writer.write_all(&[0x50, 0x4b, 0x03, 0x04])?; // Signature
        self.writer.write_all(&[0x0a, 0x00])?; // Version needed (1.0)
        self.writer.write_all(&[0x00, 0x00])?; // Flags
        self.writer.write_all(&[0x00, 0x00])?; // Compression (0 = Store)
        self.writer.write_all(&[0x00, 0x00, 0x00, 0x00])?; // Time/Date (Zeroed)
        self.writer.write_all(&crc.to_le_bytes())?;
        self.writer.write_all(&size.to_le_bytes())?; // Compressed
        self.writer.write_all(&size.to_le_bytes())?; // Uncompressed
        self.writer.write_all(&(name.len() as u16).to_le_bytes())?; // Filename len
        self.writer.write_all(&[0x00, 0x00])?; // Extra field len
        self.writer.write_all(name.as_bytes())?; // Filename
        
        // Data
        self.writer.write_all(data)?;

        self.entries.push(ZipEntry {
            name: name.to_string(),
            crc32: crc,
            compressed_size: size,
            uncompressed_size: size,
            offset,
        });

        Ok(())
    }

    fn finish(&mut self) -> io::Result<()> {
        let dir_start = self.writer.stream_position()? as u32;

        // Central Directory
        for entry in &self.entries {
            self.writer.write_all(&[0x50, 0x4b, 0x01, 0x02])?; // Signature
            self.writer.write_all(&[0x0a, 0x00])?; // Version made by
            self.writer.write_all(&[0x0a, 0x00])?; // Version needed
            self.writer.write_all(&[0x00, 0x00])?; // Flags
            self.writer.write_all(&[0x00, 0x00])?; // Compression (0 = Store)
            self.writer.write_all(&[0x00, 0x00, 0x00, 0x00])?; // Time/Date
            self.writer.write_all(&entry.crc32.to_le_bytes())?;
            self.writer.write_all(&entry.compressed_size.to_le_bytes())?;
            self.writer.write_all(&entry.uncompressed_size.to_le_bytes())?;
            self.writer.write_all(&(entry.name.len() as u16).to_le_bytes())?;
            self.writer.write_all(&[0x00, 0x00])?; // Extra field len
            self.writer.write_all(&[0x00, 0x00])?; // Comment len
            self.writer.write_all(&[0x00, 0x00])?; // Disk start
            self.writer.write_all(&[0x00, 0x00])?; // Internal attrs
            self.writer.write_all(&[0x00, 0x00, 0x00, 0x00])?; // External attrs
            self.writer.write_all(&entry.offset.to_le_bytes())?; // Rel offset
            self.writer.write_all(entry.name.as_bytes())?;
        }

        let dir_end = self.writer.stream_position()? as u32;
        let dir_size = dir_end - dir_start;
        let count = self.entries.len() as u16;

        // End of Central Directory Record
        self.writer.write_all(&[0x50, 0x4b, 0x05, 0x06])?;
        self.writer.write_all(&[0x00, 0x00])?; // Disk number
        self.writer.write_all(&[0x00, 0x00])?; // Disk with CD
        self.writer.write_all(&count.to_le_bytes())?; // Entries on disk
        self.writer.write_all(&count.to_le_bytes())?; // Total entries
        self.writer.write_all(&dir_size.to_le_bytes())?;
        self.writer.write_all(&dir_start.to_le_bytes())?;
        self.writer.write_all(&[0x00, 0x00])?; // Comment len

        Ok(())
    }
}

fn crc32_simple(data: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFFu32;
    for byte in data {
        crc ^= *byte as u32;
        for _ in 0..8 {
            if (crc & 1) != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc = crc >> 1;
            }
        }
    }
    !crc
}

// --- STL Parsing ---

struct Vertex { x: f32, y: f32, z: f32 }

fn u32_from_le_slice(s: &[u8]) -> u32 {
    ((s[0] as u32) << 0) | ((s[1] as u32) << 8) | ((s[2] as u32) << 16) | ((s[3] as u32) << 24)
}

fn f32_from_le_slice(s: &[u8]) -> f32 {
    let bits = u32_from_le_slice(s);
    f32::from_bits(bits)
}

fn parse_stl(path: &Path, current_file_idx: usize, total_files: usize) -> io::Result<Vec<Vertex>> {
    let mut file = File::open(path)?;
    
    let mut header = [0u8; 80];
    file.read_exact(&mut header)?;
    
    let mut count_buf = [0u8; 4];
    file.read_exact(&mut count_buf)?;
    let count = u32_from_le_slice(&count_buf) as usize;

    let mut vertices = Vec::with_capacity(count * 3);
    let mut buf = [0u8; 50]; 

    println!("PROGRESS: Parsing STL {}/{}: 0%", current_file_idx + 1, total_files);
    for i in 0..count {
        file.read_exact(&mut buf)?;
        
        vertices.push(Vertex {
            x: f32_from_le_slice(&buf[12..16]),
            y: f32_from_le_slice(&buf[16..20]),
            z: f32_from_le_slice(&buf[20..24]),
        });
        vertices.push(Vertex {
            x: f32_from_le_slice(&buf[24..28]),
            y: f32_from_le_slice(&buf[28..32]),
            z: f32_from_le_slice(&buf[32..36]),
        });
        vertices.push(Vertex {
            x: f32_from_le_slice(&buf[36..40]),
            y: f32_from_le_slice(&buf[40..44]),
            z: f32_from_le_slice(&buf[44..48]),
        });

        if i % (count / 20).max(1) == 0 {
            let percent = (i as f64 / count as f64 * 100.0) as usize;
            println!("PROGRESS: Parsing STL {}/{}: {}%", current_file_idx + 1, total_files, percent);
        }
    }
    println!("PROGRESS: Parsing STL {}/{}: 100%", current_file_idx + 1, total_files);

    Ok(vertices)
}

struct ProcessedObject {
    id: u32,
    name: String,
    unique_verts: Vec<(f32, f32, f32)>,
    indices: Vec<u32>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let mut inputs = Vec::new();
    let mut output_path = String::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--input" => {
                if i + 1 < args.len() {
                    inputs.push(args[i+1].clone());
                    i += 1;
                }
            },
            "--output" => {
                if i + 1 < args.len() {
                    output_path = args[i+1].clone();
                    i += 1;
                }
            },
            _ => {}
        }
        i += 1;
    }

    if inputs.is_empty() || output_path.is_empty() {
        eprintln!("Usage: stl23mf --input <in> [--input <in2> ...] --output <out>");
        std::process::exit(1);
    }

    let mut processed_objects = Vec::new();
    let mut next_obj_id = 1u32;

    for (idx, input_path) in inputs.iter().enumerate() {
        let path = Path::new(input_path);
        let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        let vertices = parse_stl(path, idx, inputs.len())?;
        
        use std::collections::BTreeMap;
        let mut unique_map = BTreeMap::new();
        let mut indices = Vec::with_capacity(vertices.len());
        let mut next_vert_id = 0u32;
        
        let total_v = vertices.len();
        println!("PROGRESS: Optimizing Mesh {}/{}: 0%", idx + 1, inputs.len());
        
        for (vii, v) in vertices.iter().enumerate() {
            let key = (v.x.to_bits(), v.y.to_bits(), v.z.to_bits());
            if let Some(&id) = unique_map.get(&key) {
                indices.push(id);
            } else {
                unique_map.insert(key, next_vert_id);
                indices.push(next_vert_id);
                next_vert_id += 1;
            }
            
            if vii % (total_v / 20).max(1) == 0 {
                let percent = (vii as f64 / total_v as f64 * 100.0) as usize;
                println!("PROGRESS: Optimizing Mesh {}/{}: {}%", idx + 1, inputs.len(), percent);
            }
        }
        println!("PROGRESS: Optimizing Mesh {}/{}: 100%", idx + 1, inputs.len());

        let mut ordered_verts = vec![(0f32,0f32,0f32); unique_map.len()];
        for ((xb, yb, zb), id) in unique_map {
            ordered_verts[id as usize] = (f32::from_bits(xb), f32::from_bits(yb), f32::from_bits(zb));
        }

        processed_objects.push(ProcessedObject {
            id: next_obj_id,
            name: filename,
            unique_verts: ordered_verts,
            indices,
        });
        next_obj_id += 1;
    }

    // --- Generate 3mf XML ---
    
    // 1. 3dmodel.model
    let mut xml = String::with_capacity(1024 * 1024); // Preallocate 1MB
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<model unit=\"millimeter\" xml:lang=\"en-US\" xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">\n");
    xml.push_str("  <resources>\n");
    
    for obj in &processed_objects {
        xml.push_str(&format!("    <object id=\"{}\" type=\"model\">\n", obj.id));
        xml.push_str("      <mesh>\n");
        
        xml.push_str("        <vertices>\n");
        for (x, y, z) in &obj.unique_verts {
            xml.push_str(&format!("          <vertex x=\"{}\" y=\"{}\" z=\"{}\" />\n", x, y, z));
        }
        xml.push_str("        </vertices>\n");

        xml.push_str("        <triangles>\n");
        for chunk in obj.indices.chunks(3) {
            xml.push_str(&format!("          <triangle v1=\"{}\" v2=\"{}\" v3=\"{}\" />\n", chunk[0], chunk[1], chunk[2]));
        }
        xml.push_str("        </triangles>\n");
        
        xml.push_str("      </mesh>\n");
        xml.push_str("    </object>\n");
    }
    
    xml.push_str("  </resources>\n");
    xml.push_str("  <build>\n");
    for obj in &processed_objects {
        xml.push_str(&format!("    <item objectid=\"{}\" />\n", obj.id));
    }
    xml.push_str("  </build>\n");
    xml.push_str("</model>\n");

    // 2. Metadata/model_settings.config (Bambu/Orca Slicer compatibility)
    let mut config_xml = String::new();
    config_xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    config_xml.push_str("<config>\n");
    
    let mut part_id_counter = 1;
    
    // Config Objects
    for obj in &processed_objects {
        config_xml.push_str(&format!("  <object id=\"{}\">\n", obj.id));
        config_xml.push_str(&format!("    <metadata key=\"name\" value=\"{}\"/>\n", obj.name));
        config_xml.push_str("    <metadata key=\"extruder\" value=\"1\"/>\n");
        config_xml.push_str(&format!("    <part id=\"{}\" subtype=\"normal_part\">\n", part_id_counter));
        config_xml.push_str(&format!("      <metadata key=\"name\" value=\"{}\"/>\n", obj.name));
        config_xml.push_str("      <metadata key=\"matrix\" value=\"1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 1\"/>\n");
        config_xml.push_str("    </part>\n");
        config_xml.push_str("  </object>\n");
        part_id_counter += 1;
    }

    // Plate Definition (All on Plate 1 for now)
    config_xml.push_str("  <plate>\n");
    config_xml.push_str("    <metadata key=\"plater_id\" value=\"1\"/>\n");
    config_xml.push_str("    <metadata key=\"plater_name\" value=\"\"/>\n");
    config_xml.push_str("    <metadata key=\"locked\" value=\"false\"/>\n");
    
    for obj in &processed_objects {
        config_xml.push_str("    <model_instance>\n");
        config_xml.push_str(&format!("      <metadata key=\"object_id\" value=\"{}\"/>\n", obj.id));
        config_xml.push_str("      <metadata key=\"instance_id\" value=\"0\"/>\n");
        config_xml.push_str(&format!("      <metadata key=\"identify_id\" value=\"{}\"/>\n", 100 + obj.id)); // Simple unique ID
        config_xml.push_str("    </model_instance>\n");
    }
    config_xml.push_str("  </plate>\n");

    // Assemble (Transforms)
    config_xml.push_str("  <assemble>\n");
    for obj in &processed_objects {
         config_xml.push_str(&format!("   <assemble_item object_id=\"{}\" instance_id=\"0\" transform=\"1 0 0 0 1 0 0 0 1 0 0 0\" offset=\"0 0 0\" />\n", obj.id));
    }
    config_xml.push_str("  </assemble>\n");
    
    config_xml.push_str("</config>\n");


    // --- Write Zip ---
    println!("Writing 3MF: {}", output_path);
    let f = File::create(output_path)?;
    let mut zip = ZipWriter::new(BufWriter::new(f)); 

    zip.add_file("[Content_Types].xml", br#"<?xml version="1.0" encoding="UTF-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/>
  <Default Extension="config" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/> 
</Types>"#)?;
    // Note: Added .config extension just in case, though usually implicit or ignored. 

    zip.add_file("_rels/.rels", br#"<?xml version="1.0" encoding="UTF-8"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Target="/3D/3dmodel.model" Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/>
</Relationships>"#)?;

    zip.add_file("3D/3dmodel.model", xml.as_bytes())?;
    zip.add_file("Metadata/model_settings.config", config_xml.as_bytes())?;
    
    zip.finish()?;
    
    println!("Done.");
    Ok(())
}
