#!/usr/bin/env bash
# WSL2 wrapper: see pmd3_wsl.py
set -euo pipefail
script_dir="$(cd "$(dirname "$0")" && pwd)"
exec uvx --managed-python --with pymobiledevice3 python "$script_dir/pmd3_wsl.py" "$@"