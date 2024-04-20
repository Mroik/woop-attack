FROM rust:1.77.2-slim
COPY . /app
WORKDIR /app
RUN apt update -y
RUN apt install -y npm
RUN npm install -g typescript
RUN cargo b -r
ENTRYPOINT ["cargo", "r", "-r", "conf.toml"]
