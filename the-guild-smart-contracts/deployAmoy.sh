#!/usr/bin/env bash
set -euo pipefail

if [[ -f .env ]]; then
  # shellcheck disable=SC1091
  source .env
fi

if [[ -z "${AMOY_RPC_URL:-}" ]]; then
  echo "AMOY_RPC_URL env var is required" >&2
  exit 1
fi

if [[ -z "${PRIVATE_KEY:-}" ]]; then
  echo "PRIVATE_KEY env var is required for non-interactive deployment" >&2
  exit 1
fi

forge script --chain amoy script/FullDeploymentScript.s.sol:FullDeploymentScript \
  --rpc-url "$AMOY_RPC_URL" \
  --private-key "$PRIVATE_KEY" \
  --broadcast --verify -vvvv
