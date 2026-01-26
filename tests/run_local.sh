#!/bin/bash
set -e

# Configuration
export TEST_SOURCE="../test_data/Loose Files STL_and_jpg"
export INPUT_DIR="../playground/input"
export OUTPUT_DIR="../playground/output"
export STAGING_DIR="../playground/staging"
export API_URL="http://localhost:6767/api"

# 1. Setup Venv
if [ ! -d ".venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv .venv
fi

source .venv/bin/activate

# 2. Install Dependencies
echo "Installing test dependencies..."
pip install behave requests > /dev/null

# 3. Clean environment using manage.sh (Docker reset)
# Note: The tests expect a running container but clean dirs. 
# We should probably respect the "running container" state.
# But for a reliable test run, maybe restarting is cleaner?
# The feature files have "Given the processor is running".
# Steps just check API status.

# 4. Cleanup and Run Behave
echo "Cleaning output..."
rm -rf $OUTPUT_DIR/* || true

echo "Running Cucumber Tests..."
behave bdd $@ --format pretty
