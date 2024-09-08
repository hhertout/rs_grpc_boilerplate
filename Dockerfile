FROM rust:1.80.0-alpine3.20

RUN apk add --no-cache \
    protobuf-dev \
    openssl \
    gcc \
    musl-dev

# Install cargo-watch
RUN cargo install cargo-watch

WORKDIR /app

COPY . .

CMD [ "cargo", "watch", "-x", "run" ]