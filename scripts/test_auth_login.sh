#!/usr/bin/env bash
set -euo pipefail

# Test the /auth/login flow and create a profile using the returned JWT.
# Requirements: curl, node, npm. Installs ethers locally into /tmp by default.
#
# Inputs (env):
#   PUBLIC_ADDRESS (required) - wallet address
#   PRIVATE_KEY    (required) - wallet private key (0x-prefixed)
#   API_URL        (optional) - defaults to http://localhost:3001
#   PROFILE_NAME   (optional) - defaults to "Shell Tester"
#   PROFILE_DESC   (optional) - defaults to "Created via script"
#   PROFILE_AVATAR (optional) - defaults to null

API_URL="${API_URL:-http://localhost:3001}"
ADDRESS="${PUBLIC_ADDRESS:-}"
PRIVATE_KEY="${PRIVATE_KEY:-}"

# If not provided via env, prompt interactively (input hidden for private key).
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

PROFILE_NAME="${PROFILE_NAME:-Shell Tester}"
PROFILE_DESC="${PROFILE_DESC:-Created via script}"
PROFILE_AVATAR="${PROFILE_AVATAR:-null}"

# Ensure we have ethers available without polluting the repo.
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
# Parse nonce safely
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

echo "Logging in..."
login_tmp="$(mktemp)"
http_status="$(curl -sS -o "${login_tmp}" -w "%{http_code}" -X POST \
  -H "x-eth-address: ${ADDRESS}" \
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

echo "JWT: ${jwt}"

echo "Creating profile..."
profile_payload=$(cat <<EOF
{
  "name": "${PROFILE_NAME}",
  "description": "${PROFILE_DESC}",
  "avatar_url": ${PROFILE_AVATAR}
}
EOF
)

create_tmp="$(mktemp)"
create_status="$(curl -sS -o "${create_tmp}" -w "%{http_code}" -X POST \
  -H "Authorization: Bearer ${jwt}" \
  -H "Content-Type: application/json" \
  -d "${profile_payload}" \
  "${API_URL}/profiles")"
create_resp="$(cat "${create_tmp}")"
rm -f "${create_tmp}"

echo "Create profile HTTP ${create_status}: ${create_resp}"
if [[ "${create_status}" != "201" && "${create_status}" != "200" ]]; then
  echo "Profile creation failed with status ${create_status}"
  exit 1
fi

echo "Done."

