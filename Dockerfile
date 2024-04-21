FROM rust:1.77.2-slim
COPY . /app
RUN apt update -y
RUN apt install -y npm
RUN npm install -g typescript
WORKDIR /app/static
RUN tsc -p tsconfig.json
WORKDIR /app
RUN cargo b -r
ENTRYPOINT ["cargo", "r", "-r", "conf.toml"]
