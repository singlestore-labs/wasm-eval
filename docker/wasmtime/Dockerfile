ARG REGISTRY
FROM $REGISTRY/base-wasm

ARG WASMTIME_VERSION=v0.26.1

WORKDIR /usr/src/

RUN git clone --recurse-submodules https://github.com/bytecodealliance/wasmtime.git

WORKDIR /usr/src/wasmtime

RUN git checkout $WASMTIME_VERSION && \
  cargo build --release
