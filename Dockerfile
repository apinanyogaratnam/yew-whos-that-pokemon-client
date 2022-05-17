FROM rust:1.59-alpine

COPY . .

RUN cargo build --release

CMD ["cargo", "run"]
