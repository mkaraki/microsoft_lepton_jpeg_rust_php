FROM rust:1-bookworm AS builder

RUN apt-get update && apt-get install -y \
    libclang-dev \
    php-dev \
    && rm -rf /var/lib/apt/lists/*

COPY . /usr/src/app
WORKDIR /usr/src/app

RUN cargo build --release

FROM php:8.3

RUN export PHP_EXTENSION_DIR=$(php-config --extension-dir)
RUN export PHP_INI_DIR=$(php-config --ini-dir)

COPY --from=builder /usr/src/app/target/release/libmicrosoft_lepton_jpeg_rust_php.so $PHP_EXTENSION_DIR/ms-lepton.so
COPY php.conf.d/20-ms-lepton.ini $PHP_INI_DIR/20-ms-lepton.ini