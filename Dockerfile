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
    ~/.cargo/env; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Собираем проект и запускаем
RUN cargo install --path .
CMD ["test"]