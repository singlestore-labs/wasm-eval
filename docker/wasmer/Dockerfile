ARG REGISTRY
FROM $REGISTRY/base-wasm

ARG WASMER_VERSION=0.11.0

RUN set -x && \
  # wasmer
  wget -q https://github.com/wasmerio/wasmer/releases/download/${WASMER_VERSION}/wasmer-linux-amd64.tar.gz && \
  mkdir /opt/wasmer && tar xf wasmer-linux-amd64.tar.gz -C /opt/wasmer && \
  rm -f wasmer-linux-amd64.tar.gz && \
  apt-get autoremove -y && \
  rm -rf /var/lib/apt/lists/* && \
  echo 'PATH=/opt/wasmer/bin:$PATH' >> /etc/profile

ENV PATH /opt/wasmer/bin:$PATH
