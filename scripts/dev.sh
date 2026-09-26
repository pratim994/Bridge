#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BACKEND_DIR="$ROOT_DIR/backend"
FRONTEND_DIR="$ROOT_DIR/frontend"

usage() {
  cat <<'EOF'
Usage: ./scripts/dev.sh [--check]

Runs the local development stack for the game server.

Options:
  --check   Validate the environment without starting the servers.
  -h, --help Show this help output.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ "${1:-}" == "--check" ]]; then
  if [[ -f "$ROOT_DIR/.venv/bin/activate" ]]; then
    # shellcheck disable=SC1091
    . "$ROOT_DIR/.venv/bin/activate"
  else
    echo "Creating Python virtual environment..."
    python3 -m venv "$ROOT_DIR/.venv"
    # shellcheck disable=SC1091
    . "$ROOT_DIR/.venv/bin/activate"
  fi

  cd "$BACKEND_DIR"
  python manage.py check

  cd "$FRONTEND_DIR"
  cargo check --quiet
  echo "Development environment looks healthy."
  exit 0
fi

if [[ -f "$ROOT_DIR/.venv/bin/activate" ]]; then
  # shellcheck disable=SC1091
  . "$ROOT_DIR/.venv/bin/activate"
else
  echo "Creating Python virtual environment..."
  python3 -m venv "$ROOT_DIR/.venv"
  # shellcheck disable=SC1091
  . "$ROOT_DIR/.venv/bin/activate"
fi

cd "$BACKEND_DIR"
python manage.py migrate --noinput

if ! command -v trunk >/dev/null 2>&1; then
  echo "Installing trunk..."
  cargo install trunk --locked
fi

cd "$ROOT_DIR"
trap 'kill "$BACKEND_PID" 2>/dev/null || true' EXIT

echo "Starting Django backend at http://127.0.0.1:8000"
python "$BACKEND_DIR/manage.py" runserver 127.0.0.1:8000 &
BACKEND_PID=$!

sleep 2

cd "$FRONTEND_DIR"
echo "Starting frontend at http://127.0.0.1:8080"
trunk serve --port 8080
