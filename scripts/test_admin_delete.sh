#!/usr/bin/env bash
set -euo pipefail

# Test the /admin/profiles/:address delete endpoint.
# Requirements: curl, node, npm. Installs ethers locally into /tmp by default.
#
# Inputs (env):
#   ADMIN_ADDRESS    (required) - admin wallet address
#   ADMIN_PRIVATE_KEY (required) - admin wallet private key (0x-prefixed)
#   TARGET_ADDRESS   (required) - target profile address to delete
#   API_URL          (optional) - defaults to http://localhost:3001

API_URL="${API_URL:-http://localhost:3001}"
ADMIN_ADDRESS="${ADMIN_ADDRESS:-}"
ADMIN_PRIVATE_KEY="${ADMIN_PRIVATE_KEY:-}"
TARGET_ADDRESS="${TARGET_ADDRESS:-}"

# If not provided via env, prompt interactively
if [[ -z "${ADMIN_ADDRESS}" ]]; then
  read -r -p "Enter ADMIN_ADDRESS (0x...): " ADMIN_ADDRESS
fi
if [[ -z "${ADMIN_PRIVATE_KEY}" ]]; then
  read -r -s -p "Enter ADMIN_PRIVATE_KEY (0x..., hidden): " ADMIN_PRIVATE_KEY
  echo
fi
if [[ -z "${TARGET_ADDRESS}" ]]; then
  read -r -p "Enter TARGET_ADDRESS to delete (0x...): " TARGET_ADDRESS
fi

if [[ -z "${ADMIN_ADDRESS}" || -z "${ADMIN_PRIVATE_KEY}" || -z "${TARGET_ADDRESS}" ]]; then
  echo "ADMIN_ADDRESS, ADMIN_PRIVATE_KEY and TARGET_ADDRESS are required. Aborting."
  exit 1
fi

# Ensure we have ethers available
TOOLS_DIR="${TOOLS_DIR:-/tmp/theguildgenesis-login}"
export NODE_PATH="${TOOLS_DIR}/node_modules${NODE_PATH:+:${NODE_PATH}}"
export PATH="${TOOLS_DIR}/node_modules/.bin:${PATH}"
if ! node -e "require('ethers')" >/dev/null 2>&1; then
  echo "Installing ethers@6 to ${TOOLS_DIR}..."
  mkdir -p "${TOOLS_DIR}"
  npm install --prefix "${TOOLS_DIR}" ethers@6 >/dev/null
fi

echo "Fetching nonce for admin ${ADMIN_ADDRESS}..."
nonce_resp="$(curl -sS "${API_URL}/auth/nonce/${ADMIN_ADDRESS}")"
echo "Nonce response: ${nonce_resp}"

# Parse nonce
nonce="$(RESP="${nonce_resp}" python3 - <<'PY'
import json, os
data = json.loads(os.environ["RESP"])
print(data["nonce"])
PY
)"
if [[ -z "${nonce}" ]]; then
  echo "Failed to parse nonce from response"
  exit 1
fi

message=$'Sign this message to authenticate with The Guild.\n\nNonce: '"${nonce}"

echo "Signing nonce..."
signature="$(
  ADDRESS="${ADMIN_ADDRESS}" PRIVATE_KEY="${ADMIN_PRIVATE_KEY}" MESSAGE="${message}" \
  node - <<'NODE'
const { Wallet, hashMessage } = require('ethers');

const address = process.env.ADDRESS;
const pk = process.env.PRIVATE_KEY;
const message = process.env.MESSAGE;

if (!address || !pk || !message) {
  console.error("Missing ADDRESS, PRIVATE_KEY or MESSAGE");
  process.exit(1);
}

const wallet = new Wallet(pk);
if (wallet.address.toLowerCase() !== address.toLowerCase()) {
  console.error(`Private key does not match address. Wallet: ${wallet.address}, Provided: ${address}`);
  process.exit(1);
}

(async () => {
  const sig = await wallet.signMessage(message);
  console.log(sig);
})();
NODE
)"

echo "Signature: ${signature}"

echo "Logging in as admin..."
login_tmp="$(mktemp)"
http_status="$(curl -sS -o "${login_tmp}" -w "%{http_code}" -X POST \
  -H "x-eth-address: ${ADMIN_ADDRESS}" \
  -H "x-eth-signature: ${signature}" \
  "${API_URL}/auth/login")"
login_resp="$(cat "${login_tmp}")"
rm -f "${login_tmp}"

echo "Login HTTP ${http_status}: ${login_resp}"
if [[ "${http_status}" != "200" ]]; then
  echo "Login failed with status ${http_status}"
  exit 1
fi

jwt="$(RESP="${login_resp}" python3 - <<'PY'
import json, os
data = json.loads(os.environ["RESP"])
print(data["token"])
PY
)"
if [[ -z "${jwt}" ]]; then
  echo "Failed to parse JWT from login response"
  exit 1
fi

echo "JWT obtained: ${jwt:0:20}..."

echo "Deleting profile ${TARGET_ADDRESS} via admin endpoint..."
delete_tmp="$(mktemp)"
delete_status="$(curl -sS -o "${delete_tmp}" -w "%{http_code}" -X DELETE \
  -H "Authorization: Bearer ${jwt}" \
  "${API_URL}/admin/profiles/${TARGET_ADDRESS}")"
delete_resp="$(cat "${delete_tmp}")"
rm -f "${delete_tmp}"

echo "Delete HTTP ${delete_status}: ${delete_resp}"
if [[ "${delete_status}" == "204" ]]; then
  echo "✅ Profile deleted successfully!"
elif [[ "${delete_status}" == "404" ]]; then
  echo "⚠️  Profile not found"
elif [[ "${delete_status}" == "403" ]]; then
  echo "❌ Forbidden - address ${ADMIN_ADDRESS} is not an admin"
elif [[ "${delete_status}" == "401" ]]; then
  echo "❌ Unauthorized - authentication failed"
else
  echo "❌ Delete failed with status ${delete_status}"
  exit 1
fi

echo "Done."
