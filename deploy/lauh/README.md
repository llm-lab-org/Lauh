# lauh.llm-lab.org — deployment

A WordCraft fork rebranded as **Lauh** with Arabic/Persian script support
(hadith/Quran style gallery, bundled OFL Arabic fonts, grouped font bar),
served at **https://lauh.llm-lab.org** from the **llmlab** host
(`136.243.60.245`).

Mirrors `llm-lab-annotate/deploy/bayan/` conventions — same server, same
Traefik, same auto-deploy pattern — but much smaller: it's a pure static
WASM site served by one nginx container.

## Isolation

Nothing collides with other services on the host:

- Container name: `lauh_web`.
- Image tag: `lauh_web:local`.
- Only joins the existing shared `traefik_proxy` network for routing.
- No volumes, no exposed host ports.
- Namespaced Traefik labels (`lauh_web.*`).

## First-time install on the server

Prereqs: Docker + Compose, and a running Traefik on the `traefik_proxy`
external network with a Cloudflare DNS-01 cert resolver named `myresolver`
(same assumption as the hadithai / bayan stacks).

```bash
# 1. SSH in.
ssh root@136.243.60.245

# 2. One-liner installer.
curl -sSL https://raw.githubusercontent.com/llm-lab-org/Lauh/main/deploy/lauh/deploy.sh | bash
```

Or, if the box can't reach GitHub anonymously:

```bash
git clone https://github.com/llm-lab-org/Lauh.git /root/services/lauh
cd /root/services/lauh/deploy/lauh
bash deploy.sh
```

What the installer does:

1. Clones (or fast-forwards) the repo to `/root/services/lauh`.
2. Confirms the `traefik_proxy` network exists (bails out with a hint
   if not — never creates it, your Traefik owns it).
3. `docker compose up -d --build`.

The build takes ~5–10 minutes on first run (fetches the Rust toolchain,
wasm target, and `trunk`, then compiles the full WordCraft workspace for
`wasm32-unknown-unknown`). Subsequent rebuilds reuse the Docker layer
cache: only the final `trunk build --release` and the nginx stage rerun
when sources change.

## DNS

Point `lauh.llm-lab.org` at `136.243.60.245` (A record). The Cloudflare
DNS-01 resolver on the host's Traefik will issue the TLS cert
automatically on first request.

## Updates (auto-deploy)

Enable the 5-minute timer so new commits on `main` are picked up without
touching the server:

```bash
# One-time setup, from the install dir on the server:
cd /root/services/lauh/deploy/lauh
install -m 644 systemd/lauh-autodeploy.service /etc/systemd/system/
install -m 644 systemd/lauh-autodeploy.timer   /etc/systemd/system/
systemctl daemon-reload
systemctl enable --now lauh-autodeploy.timer

# Verify:
systemctl list-timers | grep lauh
journalctl -t lauh-autodeploy -f
```

Each timer run calls `autodeploy.sh`:

- `git fetch origin main`
- If `HEAD != origin/main`, `git reset --hard` + `docker compose up -d --build`
- Logs a one-liner via `logger -t lauh-autodeploy` (visible with `journalctl -t lauh-autodeploy`)

Safe to run manually at any time:

```bash
bash /root/services/lauh/deploy/lauh/autodeploy.sh
```

## Manual update

If you prefer to run the installer by hand instead of using the timer:

```bash
cd /root/services/lauh/deploy/lauh
bash deploy.sh
```

Same effect — git pull + rebuild + restart.

## Uninstall (leaves nothing behind)

```bash
cd /root/services/lauh/deploy/lauh
docker compose down
# Remove autodeploy timer if installed:
systemctl disable --now lauh-autodeploy.timer || true
rm -f /etc/systemd/system/lauh-autodeploy.{service,timer}
systemctl daemon-reload
# Optional: remove the checkout.
# rm -rf /root/services/lauh
```

Won't touch the `traefik_proxy` network (external), other services, or
the host's Traefik.

## What's inside

- `Dockerfile` — multi-stage build. Stage 1 is `rust:1.99-slim` plus the
  `wasm32-unknown-unknown` target and `trunk`, builds `apps/wordcraft-web`
  with `CRAFT_FONTS_DIR=/src/deploy/lauh/craft-fonts` so the Arabic font
  bundle is embedded. Stage 2 is `nginx:1.27-alpine` serving the resulting
  `dist/web/` on port 8080.
- `nginx.conf` — static-site config: WASM MIME type, aggressive caching
  for hashed assets, `try_files` fallback to `index.html`.
- `docker-compose.yaml` — one service (`web` → `lauh_web`), Traefik labels
  for `Host(lauh.llm-lab.org)` on `websecure`.
- `deploy.sh` — idempotent installer (clone + build + up).
- `autodeploy.sh` — 5-minute poll + reconcile.
- `systemd/` — the oneshot service + timer that drive `autodeploy.sh`.
- `craft-fonts/` — the Arabic/Persian font bundle (manifest + .ttf/.otf
  files) that the font bar's "Quran/Hadith" and expanded Arabic section
  depend on.
