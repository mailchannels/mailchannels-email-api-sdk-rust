ARG RUST_BASE
FROM ${RUST_BASE}
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*
