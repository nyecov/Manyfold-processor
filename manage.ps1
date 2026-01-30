# Manyfold Processor Management Script (Windows)

$COMMAND = $args[0]

function Show-Help {
    Write-Host "Usage: .\manage.ps1 [start|stop|restart|logs|build|clean]"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  start   - Build and start the processor in the background"
    Write-Host "  stop    - Stop the running processor"
    Write-Host "  restart - Restart the processor"
    Write-Host "  logs    - View live logs"
    Write-Host "  build   - Force a rebuild of the Docker image"
    Write-Host "  clean   - Remove containers and temporary files"
}

if ($null -eq $COMMAND) {
    Show-Help
    exit
}

switch ($COMMAND) {
    "start" {
        Write-Host "🚀 Starting Manyfold Processor..." -ForegroundColor Cyan
        docker compose up -d --build
    }
    "stop" {
        Write-Host "🛑 Stopping Manyfold Processor..." -ForegroundColor Yellow
        docker compose down
    }
    "restart" {
        Write-Host "🔄 Restarting Manyfold Processor..." -ForegroundColor Cyan
        docker compose restart
    }
    "logs" {
        docker compose logs -f
    }
    "build" {
        Write-Host "🛠️ Rebuilding image..." -ForegroundColor Cyan
        docker compose build
    }
    "clean" {
        Write-Host "🧹 Cleaning up..." -ForegroundColor Yellow
        docker compose down -v
        Remove-Item -Path ".\temp\*" -Recurse -Force -ErrorAction SilentlyContinue
    }
    default {
        Show-Help
    }
}
