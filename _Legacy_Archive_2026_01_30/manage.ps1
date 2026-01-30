<#
.SYNOPSIS
Manyfold Processor Management Script (Windows)
#>

$Command = $args[0]

function Show-Help {
    Write-Host "Usage: .\manage.ps1 [command]"
    Write-Host ""
    Write-Host "Commands:"
    Write-Host "  start       Start the processor (background)"
    Write-Host "  stop        Stop the processor"
    Write-Host "  restart     Restart the processor"
    Write-Host "  update      Rebuild the processor (Apply code/library changes)"
    Write-Host "  logs        Follow the logs"
    Write-Host "  benchmark   Run performance benchmark"
    Write-Host "  test        Run BDD tests (requires Python + Behave)"
    Write-Host "  status      Check container status"
    Write-Host ""
}

if (-not $Command) {
    Show-Help
    exit 1
}

switch ($Command) {
    "start" {
        Write-Host "Starting Manyfold Processor..."
        docker compose up -d
    }
    "stop" {
        Write-Host "Stopping Manyfold Processor..."
        docker compose down
    }
    "restart" {
        Write-Host "Restarting..."
        docker compose restart
    }
    "update" {
        Write-Host "Updating Processor (Rebuilding Image)..."
        Write-Host "This will apply all changes to 'requirements.txt' and Python code."
        docker compose down
        docker compose build --no-cache
        docker compose up -d --force-recreate
        Write-Host "Update Complete."
    }
    "logs" {
        docker compose logs -f
    }
    "benchmark" {
        Write-Host "Running Benchmark (TestCase 2)..."
        # Check if python or python3 is available
        if (Get-Command python -ErrorAction SilentlyContinue) {
            python tests/benchmark.py
        }
        elseif (Get-Command python3 -ErrorAction SilentlyContinue) {
            python3 tests/benchmark.py
        }
        else {
            Write-Error "Python not found! Please install Python to run benchmarks locally."
        }
    }
    "test" {
        Write-Host "Running BDD Tests..."
        
        # Configuration
        $Env:TEST_SOURCE = "test_data/Loose Files STL_and_jpg"
        $Env:INPUT_DIR = "playground/input"
        $Env:OUTPUT_DIR = "playground/output"
        $Env:STAGING_DIR = "playground/staging"
        $Env:API_URL = "http://localhost:6767/api"

        # Check Python
        $PythonExe = "python"
        if (-not (Get-Command $PythonExe -ErrorAction SilentlyContinue)) {
            if (Get-Command "python3" -ErrorAction SilentlyContinue) {
                $PythonExe = "python3"
            }
            else {
                Write-Error "Python not found! Please install Python."
                exit 1
            }
        }

        # Install Dependencies (Minimal check)
        Write-Host "Ensuring test dependencies..."
        & $PythonExe -m pip install behave requests --disable-pip-version-check | Out-Null

        # Clean Output
        if (Test-Path $Env:OUTPUT_DIR) {
            Remove-Item -Path "$($Env:OUTPUT_DIR)\*" -Recurse -Force -ErrorAction SilentlyContinue
        }

        # Run Behave
        $BehaveArgs = $args[1..($args.Length - 1)]
        if (-not $BehaveArgs) {
            $BehaveTarget = "tests/bdd"
        }
        else {
            $BehaveTarget = $BehaveArgs
        }

        Write-Host "Running Behave on $BehaveTarget..."
        & $PythonExe -m behave $BehaveTarget --format pretty
    }
    "status" {
        docker ps --filter "name=manyfold-processor"
    }
    Default {
        Write-Host "Unknown command: $Command"
        Show-Help
        exit 1
    }
}
