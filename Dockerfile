FROM rust:1.60.0-slim-buster

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN trunk build --release

CMD ["trunk", "serve"]
