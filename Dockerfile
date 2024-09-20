FROM rust:1.81.0-alpine AS builder

RUN apk add npm musl-dev
RUN npm install -g typescript

COPY . /app
WORKDIR /app/static
RUN tsc -p tsconfig.json

WORKDIR /app
RUN cargo b -r
RUN cp target/release/woop-attack .
RUN cargo clean

FROM alpine:3.20.3
RUN mkdir /app
COPY --from=builder /app /app
WORKDIR /app
ENTRYPOINT ["./woop-attack", "conf.toml"]
