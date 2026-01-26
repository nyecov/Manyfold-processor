from fastapi import FastAPI, Request
from fastapi.templating import Jinja2Templates
from fastapi.staticfiles import StaticFiles
from pydantic import BaseModel
import processor.manager  # Import the module to access the global instance if needed, or better: inject it

app = FastAPI(title="Manyfold Processor UI")

# Mount Static
app.mount("/static", StaticFiles(directory="web/static"), name="static")

# Templates
templates = Jinja2Templates(directory="web/templates")

# Models
class SettingsUpdate(BaseModel):
    auto_process: bool = None
    theme: str = None

@app.get("/")
async def read_root(request: Request):
    from processor.context import manager 
    import os
    
    # Count files in staging
    try:
        staging_files = [f for f in os.listdir(manager.staging_dir) if not f.startswith('.')]
        queue_count = len(staging_files)
    except FileNotFoundError:
        queue_count = 0
    
    return templates.TemplateResponse("index.html", {
        "request": request, 
        "auto_process": manager.auto_process,
        "theme": manager.theme,
        "queue_count": queue_count,
        "staging_files": staging_files
    })

@app.get("/api/status")
async def get_status():
    from processor.context import manager
    import os
    
    # Count files in staging/input
    staging_count = 0
    input_count = 0
    try:
        staging_count = len([f for f in os.listdir(manager.staging_dir) if not f.startswith('.')])
        input_count = len([f for f in os.listdir(manager.input_dir) if not f.startswith('.')])
    except: pass
    
    return {
        "state": manager.state, # waiting, processing, locked
        "auto_process": manager.auto_process,
        "queue": {
            "staging": staging_count,
            "input": input_count
        }
    }

@app.get("/api/errors")
async def get_errors():
    from processor.context import manager
    return {"errors": manager.errors}

@app.get("/api/settings")
async def get_settings():
    from processor.context import manager
    return {
        "auto_process": manager.auto_process,
        "theme": manager.theme,
        "auto_off_when_done": manager.auto_off_when_done
    }

@app.post("/api/settings")
async def update_settings(settings: SettingsUpdate):
    from processor.context import manager
    
    if settings.auto_process is not None:
        manager.auto_process = settings.auto_process
    
    if settings.theme is not None:
        manager.theme = settings.theme
    
    return await get_settings()
        
from fastapi import BackgroundTasks

@app.post("/api/process/all")
async def process_all(background_tasks: BackgroundTasks):
    from processor.context import manager
    background_tasks.add_task(manager.process_all_loose)
    return {"status": "processing_started"}

@app.post("/api/process/{filename}")
async def process_file(filename: str, background_tasks: BackgroundTasks):
    from processor.context import manager
    # process_file returns success boolean, but in background we can't return it.
    # We should just trigger it.
    background_tasks.add_task(manager.process_file, filename)
    return {"status": "processing_queued", "message": f"{filename} queued"}

