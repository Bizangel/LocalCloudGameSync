#!/bin/bash
# Usage: lcgsync_steam_wrapper.sh <key> <command> [args...]

# Grab the first argument as the key
KEY="$1"
shift

# Exit immediately on error
set -e

# Check if key and command were provided
if [ -z "$KEY" ] || [ $# -eq 0 ]; then
    echo "Usage: $0 <key> <command> [args...]"
    exit 1
fi

# Run the pre-sync
lcgsync ui "$KEY"

# Run the command (the game)
echo "Starting game: $*"
"$@"

# Run the post-sync
lcgsync ui "$KEY" --after-game
