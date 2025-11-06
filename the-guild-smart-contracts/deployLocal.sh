#!/usr/bin/env bash
set -euo pipefail

# Usage:
#   LOCAL_PRIVATE_KEY=<hex_pk> AMOY_RPC_URL=<amoy_rpc> ./deployLocal.sh
#
# This script:
# 1) Starts an Anvil local node forking Polygon Amoy (to reuse EAS deployments)
# 2) Runs FullDeploymentScript against the local fork

if [[ ! -f .env ]]; then
  echo ".env not found. Continuing without sourcing."
else
  # shellcheck disable=SC1091
  source .env
fi

RPC_FORK_URL=${AMOY_RPC_URL:-}
if [[ -z "${RPC_FORK_URL}" ]]; then
  echo "AMOY_RPC_URL env var is required (RPC endpoint to fork)." >&2
  exit 1
fi

LOCAL_PK=${LOCAL_PRIVATE_KEY:-}
if [[ -z "${LOCAL_PK}" ]]; then
  echo "LOCAL_PRIVATE_KEY env var is required (used to sign txs on the local fork)." >&2
  exit 1
fi

PORT=${LOCAL_ANVIL_PORT:-8545}

if ! command -v anvil >/dev/null 2>&1; then
  echo "anvil not found. Install Foundry (foundryup) first: https://book.getfoundry.sh/" >&2
  exit 1
fi

echo "Starting anvil fork of Amoy on http://127.0.0.1:${PORT} ..."
LOG_FILE=".anvil_local_fork.log"
anvil \
  --fork-url "${RPC_FORK_URL}" \
  --chain-id 80002 \
  --port "${PORT}" \
  --block-time 1 \
  --no-storage-caching \
  >"${LOG_FILE}" 2>&1 &
ANVIL_PID=$!
trap 'kill ${ANVIL_PID} >/dev/null 2>&1 || true' EXIT

# Wait until JSON-RPC responds (up to ~15s)
ATTEMPTS=30
until curl -sS -X POST \
  -H 'Content-Type: application/json' \
  --data '{"jsonrpc":"2.0","id":1,"method":"web3_clientVersion","params":[]}' \
  "http://127.0.0.1:${PORT}" >/dev/null 2>&1; do
  ATTEMPTS=$((ATTEMPTS-1))
  if [[ ${ATTEMPTS} -le 0 ]]; then
    echo "Anvil did not become ready on port ${PORT}. Logs:" >&2
    echo "---------------- anvil logs ----------------" >&2
    tail -n 200 "${LOG_FILE}" >&2 || true
    echo "-------------------------------------------" >&2
    exit 1
  fi
  sleep 0.5
done

echo "Running FullDeploymentScript against local fork ..."
forge script script/FullDeploymentScript.s.sol:FullDeploymentScript \
  --rpc-url "http://127.0.0.1:${PORT}" \
  --private-key "${LOCAL_PK}" \
  --broadcast -vvvv

echo "Done. Anvil (PID ${ANVIL_PID}) will be stopped now."

