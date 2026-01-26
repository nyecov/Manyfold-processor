import asyncio
import logging
import uvicorn
import signal
import sys
from processor.watcher import WatchdogService
from web.app import app
from processor.context import manager

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[logging.StreamHandler(sys.stdout)]
)
logger = logging.getLogger(__name__)

# Global instances (watchdog uses singleton manager)
watchdog = WatchdogService(manager)

async def start_watchdog():
    """Starts the filesystem watchdog in a separate thread/loop"""
    logger.info("Starting Watchdog Service...")
    watchdog.start()

def print_startup_banner():
    banner = r"""
  __  __                  __       _     _ 
 |  \/  | __ _ _ __  _   / _| ___ | | __| |
 | |\/| |/ _` | '_ \| | | |_ / _ \| |/ _` |
 | |  | | (_| | | | | |_|  _| (_) | | (_| |
 |_|  |_|\__,_|_| |_|\__, |_| \___/|_|\__,_|
                     |___/                  
    """
    logger.info(banner)
    logger.info("==================================================")
    logger.info("       MANYFOLD PROCESSOR - SYSTEM ONLINE         ")
    logger.info("==================================================")
    logger.info(f" Environment: Docker")
    logger.info(f" Role:        Processor & Watchdog")
    logger.info(f" Feature:     Rust Optimizer [ACTIVE]")
    logger.info(f" Feature:     Multi-Plate Support [ACTIVE]")
    logger.info("==================================================")

async def main():
    """Main entry point combining Web Server and Watchdog"""
    print_startup_banner()

    # Start Watchdog
    watchdog.start()
    
    # Configure Uvicorn config
    config = uvicorn.Config(
        app=app, 
        host="0.0.0.0", 
        port=6767, 
        log_level="info"
    )
    server = uvicorn.Server(config)

    # Run everything
    try:
        await server.serve()
    except asyncio.CancelledError:
        logger.info("Server cancelled, shutting down...")
    finally:
        logger.info("Stopping Watchdog...")
        watchdog.stop()

if __name__ == "__main__":
    # Handle SIGTERM/SIGINT for graceful shutdown
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    try:
        loop.run_until_complete(main())
    except KeyboardInterrupt:
        pass
    finally:
        loop.close()
