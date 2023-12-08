FROM rust:latest AS rust-build

WORKDIR /usr/src

RUN USER=root cargo new simple-websocket
WORKDIR /usr/src/simple-websocket
COPY ./Cargo.toml ./Cargo.lock ./
RUN rm -rf src/
RUN cargo build --release
COPY ./src/ ./
RUN cargo build --release




FROM alpine:latest

WORKDIR /simple-websocket

COPY --from=rust-build /usr/src/simple-websocket/target/release/simple-websocket /usr/local/bin/simple-websocket

CMD simple-websocket