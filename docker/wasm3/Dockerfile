ARG REGISTRY
FROM $REGISTRY/base-wasm

ARG WASM3_VERSION=0.11.0

WORKDIR /usr/src/

RUN git clone https://github.com/wasm3/wasm3

WORKDIR /usr/src/wasm3/build

# RUN cmake .. && \
#     make -j8

# Clang target-feature option          WABT option                        Chromium --js-flags option
# ----------------------------------------------------------------------------------------------------------------
# +multivalue                          --enable-multi-value               --experimental-wasm-mv
# +tail-call                           --enable-tail-call                 --experimental-wasm-return-call
# +bulk-memory                         --enable-bulk-memory               --experimental-wasm-bulk-memory
# +nontrapping-fptoint                 --enable-saturating-float-to-int   --experimental-wasm-sat-f2i-conversions
# +sign-ext                            --enable-sign-extension            --experimental-wasm-se
# +simd128, +unimplemented-simd128     --enable-simd                      --experimental-wasm-simd
