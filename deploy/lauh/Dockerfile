# syntax=docker/dockerfile:1.6
#
# Multi-stage build for lauh.llm-lab.org:
#   1. `builder`  — Rust 1.99 + wasm32 target + trunk → `apps/wordcraft-web` → dist/web/
#   2. `runtime`  — nginx:alpine serving the static dist on port 8080
#
# The build input `CRAFT_FONTS_DIR=/src/deploy/lauh/craft-fonts` embeds the Arabic/Persian font
# bundle that the font bar's "Quran/Hadith" section depends on. The directory layout matches
# `crates/fonts/build.rs`: `$CRAFT_FONTS_DIR/fonts/manifest.txt` + the referenced .ttf/.otf files.

FROM rust:1.99-slim-bookworm AS builder
RUN apt-get update \
 && apt-get install -y --no-install-recommends curl ca-certificates pkg-config libssl-dev build-essential \
 && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
WORKDIR /src
COPY . .
ENV CRAFT_FONTS_DIR=/src/deploy/lauh/craft-fonts
# trunk writes to `dist` at the workspace root per the project's Trunk.toml (`dist = "../../dist/web"`).
RUN cd apps/wordcraft-web && trunk build --release

FROM nginx:1.27-alpine AS runtime
COPY --from=builder /src/dist/web /usr/share/nginx/html
COPY deploy/lauh/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 8080
