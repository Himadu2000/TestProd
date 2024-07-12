FROM docker.io/debian:bookworm-slim

WORKDIR /app

## copy the main binary
COPY ./main ./main

RUN apt update && apt install curl -y

EXPOSE 8000

HEALTHCHECK --interval=10s --start-period=20s CMD curl -f http://localhost:8080/graphql || exit 1

CMD ./main
