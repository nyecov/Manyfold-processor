mod hal;
mod web;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();
    log::info!("Starting Manyfold Processor v0.3.0");

    // Initialize Hardware Abstraction Layer
    let (_image_processor, _inference_engine) = hal::select_hal();

    // Start the web server in a background task
    let web_handle = tokio::spawn(async {
        if let Err(e) = web::start_web_server().await {
            log::error!("Web server failed: {}", e);
        }
    });

    log::info!("Manyfold Processor is running. Press Ctrl+C to stop.");

    // Wait for the web server (or other tasks) to finish
    let _ = tokio::join!(web_handle);

    Ok(())
}
