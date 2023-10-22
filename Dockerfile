FROM rust:1.73.0 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM --platform=linux/amd64 gcr.io/distroless/cc
RUN apt update && apt upgrade
RUN apt install -y openssl
COPY --from=build-env /app/target/release/chat-ws /
CMD ["./chat-ws"]
