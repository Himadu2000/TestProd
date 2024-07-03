FROM docker.io/rust:1-slim-bookworm AS build

## cargo package name: customize here or provide via --build-arg
ARG pkg=rocket-app

WORKDIR /build

COPY . .

RUN apt update && apt install openssl libssl-dev -y

RUN cargo build --release

################################################################################

