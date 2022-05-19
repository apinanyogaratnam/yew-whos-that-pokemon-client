FROM rust:1.60.0-slim-buster

WORKDIR /app

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked --version 0.15.0 trunk
RUN trunk build --release

EXPOSE 8080

CMD ["trunk", "serve", "--release"]
