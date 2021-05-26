ARG REGISTRY
FROM $REGISTRY/base-wasm

ARG WASMTIME_VERSION=0.11.0

RUN set -x && \
  # wasmtime
  curl https://wasmtime.dev/install.sh -sSf | bash && \
  apt-get remove --purge && \
  apt-get autoremove -y && \
  rm -rf /var/lib/apt/lists/* && \
  echo 'PATH=/opt/wasmer/bin:$PATH' >> /etc/profile

ENV PATH /opt/wasmer/bin:$PATH
