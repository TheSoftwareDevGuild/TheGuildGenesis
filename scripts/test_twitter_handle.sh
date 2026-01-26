#!/usr/bin/env bash
set -euo pipefail

# Test twitter_handle profile update via API:
# - Get profile
# - Update profile with twitter_handle
# Requirements: curl, node, npm. Installs ethers locally into /tmp by default.
#
# Inputs (env):
#   PUBLIC_ADDRESS  (required) - wallet address
#   PRIVATE_KEY     (required) - wallet private key (0x-prefixed)
#   API_URL         (optional) - defaults to http://localhost:3001
#   TWITTER_HANDLE  (optional) - defaults to "testhandle"

API_URL="${API_URL:-http://localhost:3001}"
ADDRESS="${PUBLIC_ADDRESS:-}"
PRIVATE_KEY="${PRIVATE_KEY:-}"

if [[ -z "${ADDRESS}" ]]; then
  read -r -p "Enter PUBLIC_ADDRESS (0x...): " ADDRESS
fi
if [[ -z "${PRIVATE_KEY}" ]]; then
  read -r -s -p "Enter PRIVATE_KEY (0x..., hidden): " PRIVATE_KEY
  echo
fi
if [[ -z "${ADDRESS}" || -z "${PRIVATE_KEY}" ]]; then
  echo "PUBLIC_ADDRESS and PRIVATE_KEY are required. Aborting."
  exit 1
fi

TWITTER_HANDLE="${TWITTER_HANDLE:-testhandle}"

# Ensure we have ethers available
TOOLS_DIR="${TOOLS_DIR:-/tmp/theguildgenesis-login}"
export NODE_PATH="${TOOLS_DIR}/node_modules${NODE_PATH:+:${NODE_PATH}}"
export PATH="${TOOLS_DIR}/node_modules/.bin:${PATH}"
if ! node -e "require('ethers')" >/dev/null 2>&1; then
  echo "Installing ethers@6 to ${TOOLS_DIR}..."
  mkdir -p "${TOOLS_DIR}"
  npm install --prefix "${TOOLS_DIR}" ethers@6 >/dev/null
fi

echo "Fetching nonce for ${ADDRESS}..."
nonce_resp="$(curl -sS "${API_URL}/auth/nonce/${ADDRESS}")"
echo "Nonce response: ${nonce_resp}"
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
  ADDRESS="${ADDRESS}" PRIVATE_KEY="${PRIVATE_KEY}" MESSAGE="${message}" \
  node - <<'NODE'
const { Wallet } = require('ethers');
const address = process.env.ADDRESS;
const pk = process.env.PRIVATE_KEY;
const message = process.env.MESSAGE;
if (!address || !pk || !message) {
  console.error("Missing ADDRESS, PRIVATE_KEY or MESSAGE");
  process.exit(1);
}
const wallet = new Wallet(pk);
if (wallet.address.toLowerCase() !== address.toLowerCase()) {
  console.error(`Private key does not match address.`);
  process.exit(1);
}
(async () => {
  const sig = await wallet.signMessage(message);
  console.log(sig);
})();
NODE
)"

echo "Fetching current profile..."
get_tmp="$(mktemp)"
get_status="$(curl -sS -o "${get_tmp}" -w "%{http_code}" "${API_URL}/profile/${ADDRESS}")"
get_resp="$(cat "${get_tmp}")"
rm -f "${get_tmp}"
echo "GET profile HTTP ${get_status}: ${get_resp}"

update_payload=$(cat <<EOF
{
  "twitter_handle": "${TWITTER_HANDLE}"
}
EOF
)

echo "Updating profile with twitter_handle: ${TWITTER_HANDLE}..."
update_tmp="$(mktemp)"
update_status="$(curl -sS -o "${update_tmp}" -w "%{http_code}" -X PUT \
  -H "x-eth-address: ${ADDRESS}" \
  -H "x-eth-signature: ${signature}" \
  -H "Content-Type: application/json" \
  -d "${update_payload}" \
  "${API_URL}/profile")"
update_resp="$(cat "${update_tmp}")"
rm -f "${update_tmp}"
echo "PUT profile HTTP ${update_status}: ${update_resp}"

if [[ "${update_status}" == "200" ]]; then
  echo "✅ Twitter handle updated successfully!"
elif [[ "${update_status}" == "400" ]]; then
  echo "❌ Invalid twitter handle format (400)"
  exit 1
elif [[ "${update_status}" == "409" ]]; then
  echo "❌ Twitter handle already taken (409)"
  exit 1
else
  echo "❌ Unexpected status ${update_status}"
  exit 1
fi

echo "Done."
