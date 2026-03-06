#!/usr/bin/env bash
set -euo pipefail

# Test admin distribution endpoints:
# - POST /admin/distributions
# - GET /admin/distributions?distributionId=...
#
# Sequence:
# 1) get nonce
# 2) generate signature
# 3) post endpoint
# 4) list endpoint
#
# Requirements: curl, node, npm. Installs ethers locally into /tmp by default.
#
# Inputs (env):
#   ADMIN_ADDRESS      (required) - admin wallet address
#   ADMIN_PRIVATE_KEY  (required) - admin wallet private key (0x-prefixed)
#   API_URL            (optional) - defaults to http://localhost:3001
#   DISTRIBUTION_ID    (optional) - defaults to dist-<timestamp>

API_URL="${API_URL:-http://localhost:3001}"
ADMIN_ADDRESS="${ADMIN_ADDRESS:-}"
ADMIN_PRIVATE_KEY="${ADMIN_PRIVATE_KEY:-}"
DISTRIBUTION_ID="${DISTRIBUTION_ID:-dist-$(date +%s)}"

if [[ -z "${ADMIN_ADDRESS}" ]]; then
  read -r -p "Enter ADMIN_ADDRESS (0x...): " ADMIN_ADDRESS
fi
if [[ -z "${ADMIN_PRIVATE_KEY}" ]]; then
  read -r -s -p "Enter ADMIN_PRIVATE_KEY (0x..., hidden): " ADMIN_PRIVATE_KEY
  echo
fi

if [[ -z "${ADMIN_ADDRESS}" || -z "${ADMIN_PRIVATE_KEY}" ]]; then
  echo "ADMIN_ADDRESS and ADMIN_PRIVATE_KEY are required. Aborting."
  exit 1
fi

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

payload=$(cat <<EOF
{
  "distributions": [
    {
      "address": "0x1111111111111111111111111111111111111111",
      "badgeName": "Contributor",
      "distributionId": "${DISTRIBUTION_ID}"
    },
    {
      "address": "0x2222222222222222222222222222222222222222",
      "badgeName": "Reviewer",
      "distributionId": "${DISTRIBUTION_ID}"
    }
  ]
}
EOF
)

echo "Posting distributions with distributionId=${DISTRIBUTION_ID}..."
post_tmp="$(mktemp)"
post_status="$(curl -sS -o "${post_tmp}" -w "%{http_code}" -X POST \
  -H "x-eth-address: ${ADMIN_ADDRESS}" \
  -H "x-eth-signature: ${signature}" \
  -H "Content-Type: application/json" \
  -d "${payload}" \
  "${API_URL}/admin/distributions")"
post_resp="$(cat "${post_tmp}")"
rm -f "${post_tmp}"

echo "POST HTTP ${post_status}: ${post_resp}"
if [[ "${post_status}" != "201" ]]; then
  echo "❌ POST /admin/distributions failed"
  exit 1
fi

echo "Listing distributions..."
list_tmp="$(mktemp)"
list_status="$(curl -sS -o "${list_tmp}" -w "%{http_code}" \
  -H "x-eth-address: ${ADMIN_ADDRESS}" \
  -H "x-eth-signature: ${signature}" \
  "${API_URL}/admin/distributions?distributionId=${DISTRIBUTION_ID}")"
list_resp="$(cat "${list_tmp}")"
rm -f "${list_tmp}"

echo "GET HTTP ${list_status}: ${list_resp}"
if [[ "${list_status}" != "200" ]]; then
  echo "❌ GET /admin/distributions failed"
  exit 1
fi

count="$(RESP="${list_resp}" python3 - <<'PY'
import json, os
items = json.loads(os.environ["RESP"])
print(len(items) if isinstance(items, list) else 0)
PY
)"

if [[ "${count}" -lt 2 ]]; then
  echo "❌ Expected at least 2 items, got ${count}"
  exit 1
fi

echo "✅ Admin distributions flow passed (count=${count})"
