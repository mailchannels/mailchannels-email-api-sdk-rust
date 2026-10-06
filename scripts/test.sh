#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
image=${SDK_RUST_IMAGE:-rust:1.90.0-slim-bookworm@sha256:64232e656c058f4468e8d024e990acff04f0fd5a5c0a88a574dc37773d7325c9}
python "$root/scripts/tls-certificates.py" "$root/tests/tls-fixtures"
docker run --rm -v "$root:/sdk" -v visibility-rust-cargo:/usr/local/cargo/registry -w /sdk "$image" cargo fetch --locked
docker run --rm --network none -v "$root:/sdk" -v visibility-rust-cargo:/usr/local/cargo/registry -w /sdk "$image" cargo test --offline --locked --tests
