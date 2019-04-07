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
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    RUSTUP_USE_CURL=1 ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    ENV PATH="${PATH}:$HOME/.cargo/bin" \
    ENV RUSTUP_HOME="~/.rustup" \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Собираем проект и запускаем
RUN cargo install --path .
CMD ["test"]