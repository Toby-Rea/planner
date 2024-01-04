FROM rust:1.75 AS builder

WORKDIR /app

COPY . .

RUN cargo build

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

CMD ["cargo", "run"]