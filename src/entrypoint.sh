#!/bin/sh

# Default PUID/PGID if not specified
PUID=${PUID:-1000}
PGID=${PGID:-1000}

echo "Starting Manyfold Processor with PUID: $PUID, PGID: $PGID"

# In a tailored container we might create a user here, 
# but for simplicity in this MVP we assume the host maps volumes 
# with appropriate permissions or we run as root inside and rely on 
# the host to ignore ownership or we adjust umask.
# 
# Ideally, we would create a user 'appuser' with PUID:PGID 
# and switch to it using gosu/su-exec. 
# For now, we'll keep it simple as requested for the skeleton.

# Execute the main application
exec python3 main.py
