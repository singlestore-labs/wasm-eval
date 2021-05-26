ARG REGISTRY
FROM $REGISTRY/debian:stretch

ARG WASI_VERSION=12.0
ARG WABT_VERSION=1.0.23
ARG CMAKE_VERSION=3.20.2
ARG BINARYEN_VERSION=version_101
ARG RUST_VERSION=1.52.1

RUN set -x && \
  apt-get update && \
  apt-get install -y --no-install-recommends \
  curl \
  wget \
  git \
  ca-certificates \
  build-essential \
  clang \
  python3 \
  git \
  ninja-build && \
  # cleanup
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* && \
  # cmake
  wget -qO- "https://cmake.org/files/v3.20/cmake-3.20.0-linux-x86_64.tar.gz" | tar --strip-components=1 -xz -C /usr/local

RUN set -x && \
  # wasi
  wget -q https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-12/wasi-sdk_${WASI_VERSION}_amd64.deb && \
  dpkg -i wasi-sdk_${WASI_VERSION}_amd64.deb && \
  rm -f wasi-sdk_${WASI_VERSION}_amd64.deb && \
  echo 'PATH=/opt/wasi-sdk/bin:$PATH' >> /etc/profile && \
  # wabt
  git clone https://github.com/WebAssembly/wabt && \
  cd wabt && \
  git checkout $WABT_VERSION && \
  git submodule update --init && \
  make

ENV LANG C.UTF-8
ENV PATH /opt/wasi-sdk/bin:/opt/cmake/bin:$PATH

# Rust
# from: https://github.com/rust-lang/docker-rust/blob/master/1.52.1/buster/Dockerfile
ENV RUSTUP_HOME=/usr/local/rustup \
  CARGO_HOME=/usr/local/cargo \
  PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
  dpkgArch="$(dpkg --print-architecture)"; \
  case "${dpkgArch##*-}" in \
  amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='fb3a7425e3f10d51f0480ac3cdb3e725977955b2ba21c9bdac35309563b115e8' ;; \
  armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='f263e170da938888601a8e0bc822f8b40664ab067b390cf6c4fdb1d7c2d844e7' ;; \
  arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='de1dddd8213644cba48803118c7c16387838b4d53b901059e01729387679dd2a' ;; \
  i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='66c03055119cecdfc20828c95429212ae5051372513f148342758bb5d0130997' ;; \
  *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
  esac; \
  url="https://static.rust-lang.org/rustup/archive/1.24.1/${rustArch}/rustup-init"; \
  wget "$url"; \
  echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
  chmod +x rustup-init; \
  ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
  rm rustup-init; \
  chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
  rustup --version; \
  cargo --version; \
  rustc --version;