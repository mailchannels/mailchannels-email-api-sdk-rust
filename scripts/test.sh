#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
image=${SDK_RUST_IMAGE:-rust:1.90.0-slim-bookworm@sha256:64232e656c058f4468e8d024e990acff04f0fd5a5c0a88a574dc37773d7325c9}
tls=${SDK_TLS_FEATURE:-rustls}
case "$tls" in
  rustls) ;;
  native-tls)
    image=$(docker build --quiet --build-arg "RUST_BASE=$image" -f "$root/scripts/NativeTls.Dockerfile" "$root/scripts")
    ;;
  *) echo 'SDK_TLS_FEATURE must be rustls or native-tls' >&2; exit 2 ;;
esac
python "$root/scripts/tls-certificates.py" "$root/tests/tls-fixtures"
docker run --rm -v "$root:/sdk" -v visibility-rust-cargo:/usr/local/cargo/registry -w /sdk "$image" cargo fetch --locked
docker run --rm --network none -v "$root:/sdk" -v visibility-rust-cargo:/usr/local/cargo/registry -w /sdk "$image" bash -c '
set -euo pipefail
rustc --version
if [ "$1" = native-tls ]; then openssl version; fi
features=$(cargo tree --offline --locked --no-default-features --features "$1" -e features -i reqwest)
printf "%s\n" "$features"
if [[ "$1" == native-tls && "$features" == *"reqwest feature \"__rustls\""* ]] ||
   [[ "$1" == rustls && "$features" == *"reqwest feature \"__native-tls\""* ]]; then
    echo "Unexpected second reqwest TLS backend in the fixture" >&2
    exit 1
fi
cargo test --offline --locked --no-default-features --features "$1" --tests
' -- "$tls"
