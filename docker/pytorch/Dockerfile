ARG REGISTRY
FROM $REGISTRY/base-wasm

COPY requirements.txt requirements.txt

RUN apt-cache search pip
    # apt-get install python3-pip && \
    # pip3 install --upgrade pip && \
    # pip3 install --requirement requirements.txt && \
    # # cleanup
    # apt-get -y clean  && \
    # apt-get autoremove --purge  -y && \
    # rm -rf /var/lib/apt/lists/*
