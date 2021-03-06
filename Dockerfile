FROM ubuntu:latest
ADD . /code
WORKDIR /code

## Установка зависимостей
RUN apt-get update && apt-get install -y \
    libvips* \
    build-essential \
    cmake \
    libboost-dev \
    libboost-all-dev \
    curl \
    wget

## Ставим Rust и переключаемся на nightly
RUN set -eux; \
    export CARGO_HOME="$HOME/.cargo"; \ 
    export RUSTUP_HOME="$HOME/.rustup"; \
    export PATH="${PATH}:$CARGO_HOME/bin:$RUSTUP_HOME"; \
    env; \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    RUSTUP_USE_CURL=1 ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    chmod +x $CARGO_HOME/env; \
    $CARGO_HOME/env; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    apt-get -y install libvips42 libvips-dev libssl-dev; \
    cargo build; \
    mkdir /opt/test/; \
    cp target/debug/test /opt/test/app; \
    cp -r static /opt/test/; \
    ls /opt/test/;

WORKDIR /opt/test/
ENTRYPOINT ["/opt/test/app"]