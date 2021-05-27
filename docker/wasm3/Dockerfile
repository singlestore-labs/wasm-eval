ARG REGISTRY
FROM $REGISTRY/base-wasm

ARG WASM3_VERSION=0.4.9

RUN set -eux; \
  wget -q -O /opt/wasm3 https://github.com/wasm3/wasm3/releases/download/v{WASM3_VERSION}/wasm3-cosmopolitan.com && \
  chmod +x /opt/wasm3
