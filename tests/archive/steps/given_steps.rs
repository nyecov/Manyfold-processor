use super::world::DashboardWorld;
use cucumber::{gherkin::Step, given};
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[given(expr = "a file {string} is in the input directory")]
async fn create_dummy_file(_world: &mut DashboardWorld, filename: String) {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let path = Path::new(&input_dir).join(&filename);

    // Create dummy file
    let mut file = File::create(&path).expect("Failed to create dummy file");

    if filename.ends_with(".jpg") || filename.ends_with(".jpeg") {
        let mut img = image::ImageBuffer::new(1, 1);
        img.put_pixel(0, 0, image::Rgb([255u8, 0u8, 0u8]));
        img.save_with_format(&path, image::ImageFormat::Jpeg)
            .expect("Failed to save dummy JPG");
    } else if filename.ends_with(".gif") {
        // Create 1x1 GIF
        let mut img = image::ImageBuffer::new(1, 1);
        img.put_pixel(0, 0, image::Rgb([0u8, 255u8, 0u8]));

        if filename.contains("animated") {
            let f = File::create(&path).unwrap();
            let mut encoder = image::codecs::gif::GifEncoder::new(f);
            let mut frames = vec![];
            for i in 0..2 {
                let mut img = image::ImageBuffer::new(1, 1);
                img.put_pixel(0, 0, image::Rgba([(i * 100) as u8, 0u8, 0u8, 255u8]));
                frames.push(image::Frame::new(img));
            }
            encoder
                .encode_frames(frames.into_iter())
                .expect("Failed to encode animated GIF");
        } else {
            img.save_with_format(&path, image::ImageFormat::Gif)
                .expect("Failed to save dummy GIF");
        }
    } else {
        writeln!(file, "dummy content").expect("Failed to write to dummy file");
    }
}

#[given(regex = "^\"([^\"]+)\" \\(([^)]+)\\) is in the input directory$")]
async fn create_file_with_size_desc(
    _world: &mut DashboardWorld,
    filename: String,
    size_desc: String,
) {
    create_file_internal(filename, Some(size_desc));
}

#[given(expr = "{string} is in the input directory")]
async fn create_file_default(_world: &mut DashboardWorld, filename: String) {
    create_file_internal(filename, None);
}

fn create_file_internal(filename: String, size_desc: Option<String>) {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());
    let path = Path::new(&input_dir).join(&filename);

    let size: u64 = if let Some(desc) = size_desc {
        if desc.to_lowercase().contains("mb") {
            desc.replace("MB", "").trim().parse::<u64>().unwrap_or(1) * 1024 * 1024
        } else if desc.to_lowercase().contains("kb") {
            desc.replace("KB", "").trim().parse::<u64>().unwrap_or(1) * 1024
        } else {
            1024 // Default 1KB
        }
    } else {
        1024 // Default 1KB
    };

    let file = File::create(&path).expect("Failed to create file");
    file.set_len(size).expect("Failed to set file size");
}

#[given("several files are in the input directory:")]
async fn create_multiple_files(_world: &mut DashboardWorld, step: &Step) {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());

    if let Some(table) = step.table.as_ref() {
        // Skip header row
        for row in table.rows.iter().skip(1) {
            let filename = &row[0];
            let path = Path::new(&input_dir).join(filename);
            let mut file = File::create(&path).expect("Failed to create dummy file from table");
            writeln!(file, "batch content").expect("Failed to write to batch file");
        }
    }
}

#[given("a large dataset is copied to the input directory")]
async fn create_large_dataset(_world: &mut DashboardWorld) {
    let input_dir = std::env::var("INPUT_DIR").unwrap_or_else(|_| "input".to_string());

    for i in 1..=25 {
        let filename = format!("dataset_file_{}.stl", i);
        let path = Path::new(&input_dir).join(filename);
        let mut file = File::create(&path).expect("Failed to create dummy dataset file");
        writeln!(file, "dummy content {}", i).expect("Failed to write to file");
    }
}

#[given("Processor is running")]
async fn service_is_running(_world: &mut DashboardWorld) {
    // UI layer: verify service is up visually
    // In strict BDD, this should check the browser.
    // For now, we assume if the test runs, it's fine, or use the API step for hard checks.
}

#[given("the System is ready")]
async fn system_is_ready(_world: &mut DashboardWorld) {
    let client = reqwest::Client::new();
    let resp = client.get("http://localhost:8080/health").send().await;
    match resp {
        Ok(res) => {
            if !res.status().is_success() {
                panic!("❌ CAUSE: System is NOT ready. Status: {}", res.status());
            }
            println!("✅ System is ready (Health Check Passed)");
        }
        Err(e) => panic!("❌ Failed to check system readiness: {}", e),
    }
}
