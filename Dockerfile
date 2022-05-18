FROM rust:1.59-alpine

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN trunk build --release

CMD ["cargo", "run"]
