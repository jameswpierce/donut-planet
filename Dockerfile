FROM rust:alpine AS build

WORKDIR /app

COPY Cargo.toml .
COPY src/ ./src

RUN apk add musl-dev
RUN cargo build --release

FROM alpine:latest
WORKDIR /app

# COPY config.toml .
COPY templates ./templates
RUN mkdir output
RUN mkdir images

COPY --from=build /app/target/release/donut-planet /usr/bin

ENTRYPOINT ["donut-planet", "serve"]
