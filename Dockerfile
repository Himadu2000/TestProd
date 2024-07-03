FROM docker.io/rust:1-slim-bookworm AS build

## cargo package name: customize here or provide via --build-arg
ARG pkg=rocket-app

WORKDIR /build

COPY . .

RUN apt update && apt install openssl libssl-dev -y

################################################################################

FROM docker.io/debian:bookworm-slim

WORKDIR /app

## copy the main binary


## ensure the container listens globally on port 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
