#!/usr/bin/env bash
# Idempotent installer for lauh.llm-lab.org on the llmlab host (136.243.60.245).
# Safe to re-run; only creates/updates its own container and image, doesn't touch
# anything else on the host.

set -euo pipefail

REPO_URL=${REPO_URL:-https://github.com/llm-lab-org/Lauh.git}
INSTALL_DIR=${INSTALL_DIR:-/root/services/lauh}
BRANCH=${BRANCH:-main}

echo "==> lauh.llm-lab.org deploy"
echo "    repo:    $REPO_URL"
echo "    dir:     $INSTALL_DIR"
echo "    branch:  $BRANCH"

# 1. Clone or fast-forward.
if [ -d "$INSTALL_DIR/.git" ]; then
  echo "==> pulling latest $BRANCH into $INSTALL_DIR"
  git -C "$INSTALL_DIR" fetch --quiet origin "$BRANCH"
  git -C "$INSTALL_DIR" checkout --quiet "$BRANCH"
  git -C "$INSTALL_DIR" reset --hard "origin/$BRANCH"
else
  echo "==> cloning $REPO_URL"
  mkdir -p "$(dirname "$INSTALL_DIR")"
  git clone --branch "$BRANCH" "$REPO_URL" "$INSTALL_DIR"
fi

cd "$INSTALL_DIR/deploy/lauh"

# 2. Confirm the shared traefik_proxy network exists. Never create it here —
#    the host's existing Traefik owns it.
if ! docker network inspect traefik_proxy >/dev/null 2>&1; then
  echo "!!  traefik_proxy network doesn't exist on this host."
  echo "!!  Check Traefik (docker network ls | grep -i traefik) and either"
  echo "!!  update the 'name: traefik_proxy' entries in docker-compose.yaml"
  echo "!!  or point them at the right network name."
  exit 1
fi

# 3. Build + start.
docker compose up -d --build

echo
echo "==> containers:"
docker compose ps
echo
echo "==> done. Visit https://lauh.llm-lab.org"
