#!/bin/bash

# Helper script for running TheGuild batch attestation script
# Usage: ./run_batch_attestations.sh [json_file] [dry_run]
#   json_file: Path to JSON file with attestations (default: attestations-latest.json)
#   dry_run: Set to 'true' for dry run (default: false)

set -e

# Source .env file if it exists
if [ -f .env ]; then
    source .env
fi

# Parse arguments - JSON file is optional, defaults to attestations-latest.json
if [ $# -eq 0 ]; then
    # No arguments: use default JSON file
    JSON_FILE="attestations-latest.json"
    DRY_RUN="false"
elif [ $# -eq 1 ]; then
    # One argument: could be JSON file or dry_run flag
    if [ "$1" = "true" ] || [ "$1" = "false" ]; then
        # It's a dry_run flag
        JSON_FILE="attestations-latest.json"
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

# Run the script
if [ "$DRY_RUN" = "true" ]; then
    forge script script/EmitAttestationsJson.s.sol:EmitAttestationsJson \
        --rpc-url "$RPC_URL"
else
    forge script script/EmitAttestationsJson.s.sol:EmitAttestationsJson \
        --rpc-url "$RPC_URL" \
        --broadcast
fi