ARG REGISTRY
FROM $REGISTRY/base-wasm

ARG WASMER_VERSION=0.11.0

RUN set -x && \
  # wasmer
  wget -q https://github.com/wasmerio/wasmer/releases/download/${WASMER_VERSION}/wasmer-linux-amd64.tar.gz && \
  mkdir /opt/wasmer && tar xf wasmer-linux-amd64.tar.gz -C /opt/wasmer && \
  rm -f wasmer-linux-amd64.tar.gz && \
  echo 'PATH=/opt/wasmer/bin:$PATH' >> /etc/profile

ENV PATH /opt/wasmer/bin:$PATH
ENV WASMER_DIR /opt/wasmer
ENV LD_LIBRARY_PATH $WASMER_DIR/lib/:$LD_LIBRARY_PATH
