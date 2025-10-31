### Notebooks Workspace

This directory hosts Jupyter notebooks for experiments and analysis.

#### Quickstart (uv)
- Install uv: see https://docs.astral.sh/uv/
- From this folder: `bash ./setup.sh`
- Activate: `source .venv/bin/activate`
- Launch: `jupyter lab`

#### Notes
- Dependencies are defined in `pyproject.toml` and managed by uv.
- A local venv is created in `.venv`.
- Kernel registered as: "Python (TheGuild Notebooks)".

#### Tips
- Keep large data files out of git
- Use clear notebook names like 00_welcome.ipynb
- Aim for reproducibility and set random seeds when needed
