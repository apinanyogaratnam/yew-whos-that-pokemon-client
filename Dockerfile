FROM rust:1.60.0-slim-buster

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build --release

CMD ["cargo", "run"]
