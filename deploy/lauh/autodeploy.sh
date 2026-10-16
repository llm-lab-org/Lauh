#!/usr/bin/env bash
# Idempotent auto-deploy runner for lauh.llm-lab.org.
# Called every 5 minutes by the systemd timer (lauh-autodeploy.timer).
# - Fetches origin/main.
# - If the local checkout is behind, fast-forwards and rebuilds the image.
# - Restarts the container so the new build lands.
# - Emits a one-line log entry; systemd captures it via journalctl.
#
# Safe to run manually: `bash /root/services/lauh/deploy/lauh/autodeploy.sh`

set -euo pipefail

REPO_DIR=/root/services/lauh
COMPOSE_DIR=$REPO_DIR/deploy/lauh
LOG_TAG="lauh-autodeploy"

# systemd services don't inherit an interactive shell's HOME / git config.
export HOME="${HOME:-/root}"
git config --global --add safe.directory "$REPO_DIR" 2>/dev/null || true

cd "$REPO_DIR"

# 1. Discover current commit + fetch upstream.
before=$(git rev-parse HEAD)
git fetch --quiet origin main
after=$(git rev-parse origin/main)

if [ "$before" = "$after" ]; then
  logger -t "$LOG_TAG" "up-to-date at $before"
  exit 0
fi

# 2. Fast-forward.
logger -t "$LOG_TAG" "updating $before -> $after"
git reset --hard "$after"

# 3. Rebuild + restart the single web container.
cd "$COMPOSE_DIR"
logger -t "$LOG_TAG" "rebuilding lauh-web"
docker compose up -d --build

logger -t "$LOG_TAG" "deploy complete at $after"
