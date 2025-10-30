#!/usr/bin/env bash
set -euo pipefail

# Ensure uv is installed
if ! command -v uv >/dev/null 2>&1; then
  echo "Error: 'uv' not found. Install it: https://docs.astral.sh/uv/"
  exit 1
fi

# Sync environment using uv (creates .venv and installs deps from pyproject.toml)
uv sync --python 3.12

# Activate
# shellcheck source=/dev/null
source .venv/bin/activate

# Register kernel
python -m ipykernel install --user --name the-guild-notebooks --display-name "Python (TheGuild Notebooks)"

echo "Setup complete using uv. Activate with: source .venv/bin/activate"
echo "Launch Jupyter with: jupyter lab"
