#!/bin/bash

# Helper script for running TheGuild batch badge creation script
# Usage: ./run_batch_badges.sh [json_file] [dry_run]
#   json_file: Path to JSON file with badges (default: badges-latest.json)
#   dry_run: Set to 'true' for dry run (default: false)

set -e

# Source .env file if it exists
if [ -f .env ]; then
    source .env
fi

# Parse arguments - JSON file is optional, defaults to badges-latest.json
if [ $# -eq 0 ]; then
    # No arguments: use default JSON file
    JSON_FILE="badges-latest.json"
    DRY_RUN="false"
elif [ $# -eq 1 ]; then
    # One argument: could be JSON file or dry_run flag
    if [ "$1" = "true" ] || [ "$1" = "false" ]; then
        # It's a dry_run flag
        JSON_FILE="badges-latest.json"
        DRY_RUN="$1"
    else
        # It's a JSON file path
        JSON_FILE="$1"
        DRY_RUN="false"
    fi
else
    # Two arguments: JSON file and dry_run flag
    JSON_FILE="$1"
    DRY_RUN="$2"
fi

if [ ! -f "$JSON_FILE" ]; then
    echo "Error: JSON file '$JSON_FILE' not found"
    exit 1
fi

# Set JSON file path
export JSON_PATH="$JSON_FILE"

# Set dry run mode
if [ "$DRY_RUN" = "true" ]; then
    export DRY_RUN=true
    echo "Running in DRY RUN mode..."
else
    unset DRY_RUN
    echo "Running in PRODUCTION mode..."
fi

# Check for required environment variables
if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: PRIVATE_KEY environment variable not set"
    exit 1
fi

if [ -z "$RPC_URL" ]; then
    echo "Error: RPC_URL environment variable not set"
    exit 1
fi

if [ -z "$BADGE_REGISTRY_ADDRESS" ]; then
    echo "Error: BADGE_REGISTRY_ADDRESS environment variable not set"
    exit 1
fi

# Run the script
if [ "$DRY_RUN" = "true" ]; then
    forge script script/CreateBadgesFromJson.s.sol:CreateBadgesFromJson \
        --rpc-url "$RPC_URL"
else
    forge script script/CreateBadgesFromJson.s.sol:CreateBadgesFromJson \
        --rpc-url "$RPC_URL" \
        --broadcast
fi

