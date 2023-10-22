FROM rust:slim-buster as build-env
WORKDIR /app
COPY . /app
RUN apt-get update && \
  apt-get install -y pkg-config make g++ libssl-dev cmake libmariadb-dev-compat openssl && \
  rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu


FROM gcr.io/distroless/cc

COPY --from=build-env /app/target/release/chat-ws /
CMD ["./chat-ws"]
