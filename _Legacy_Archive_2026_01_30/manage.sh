#!/bin/bash
# Manyfold Processor Management Script

COMMAND=$1

function show_help {
    echo "Usage: ./manage.sh [command]"
    echo ""
    echo "Commands:"
    echo "  start       Start the processor (background)"
    echo "  stop        Stop the processor"
    echo "  restart     Restart the processor"
    echo "  update      Rebuild the processor (Apply code/library changes)"
    echo "  logs        Follow the logs"
    echo "  benchmark   Run performance benchmark"
    echo "  status      Check container status"
    echo ""
}

if [ -z "$COMMAND" ]; then
    show_help
    exit 1
fi

case "$COMMAND" in
    start)
        echo "Starting Manyfold Processor..."
        sudo docker compose up -d
        ;;
    stop)
        echo "Stopping Manyfold Processor..."
        sudo docker compose down
        ;;
    restart)
        echo "Restarting..."
        sudo docker compose restart
        ;;
    update)
        echo "Updating Processor (Rebuilding Image)..."
        echo "This will apply all changes to 'requirements.txt' and Python code."
        sudo docker compose down
        sudo docker compose build --no-cache
        sudo docker compose up -d --force-recreate
        echo "Update Complete."
        ;;
    logs)
        sudo docker compose logs -f
        ;;
    benchmark)
        echo "Running Benchmark (TestCase 2)..."
        sudo python3 tests/benchmark.py
        ;;
    status)
        sudo docker ps --filter "name=manyfold-processor"
        ;;
    *)
        echo "Unknown command: $COMMAND"
        show_help
        exit 1
        ;;
esac
