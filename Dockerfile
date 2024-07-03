FROM docker.io/rust AS build

## cargo package name: customize here or provide via --build-arg
ARG pkg=rocket-app

WORKDIR /build

COPY . .

RUN apt update && apt install openssl libssl-dev -y

RUN cargo build --release

################################################################################

