#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
mkdir -p "$root/.generated"
python "$root/codegen/prepare-spec.py" "$root/.generated/openapi.json"
uid="$(id -u):$(id -g)"
if docker info --format '{{json .SecurityOptions}}' | grep -q rootless; then uid=0:0; fi
docker run --rm --network none --user "$uid" -v "$root/codegen:/codegen:ro" -v "$root/.generated:/out" openapitools/openapi-generator-cli:v7.26.0@sha256:a304ddf1e2e5f24f68fa3153568d6174cea4959d09aa8e3db6d526fd0782326d generate -g rust -i /out/openapi.json -c /codegen/config.json -t /codegen/templates/rust -o /out/rust
cp -R "$root/.generated/rust/src/." "$root/src/"
cp -R "$root/.generated/rust/docs/." "$root/docs/"
cp "$root/.generated/rust/Cargo.toml" "$root/Cargo.toml"
docker build -f "$root/scripts/RustChecks.Dockerfile" -t visibility-rust-checks:1.90.0 "$root/scripts"
docker run --rm --network none -v "$root:/sdk" -w /sdk visibility-rust-checks:1.90.0 cargo fmt
python "$root/scripts/normalize-docs.py"
