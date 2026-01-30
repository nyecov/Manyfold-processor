from processor.manager import ProcessorManager

# Singleton instance shared by Watchdog (main) and API (web)
manager = ProcessorManager()
