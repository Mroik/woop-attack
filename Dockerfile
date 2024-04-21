FROM rust:1.77.2-slim

RUN apt update -y
RUN apt install -y npm
RUN npm install -g typescript

COPY . /app
WORKDIR /app/static
RUN tsc -p tsconfig.json

WORKDIR /app
RUN cargo b -r
RUN cp target/release/woop-attack .
RUN cargo clean

ENTRYPOINT ["./woop-attack", "conf.toml"]
