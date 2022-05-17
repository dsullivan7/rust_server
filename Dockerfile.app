FROM debian:latest

COPY ./target/release/rust_server /app/rust_server

RUN apt-get update -y
RUN apt-get install -y ca-certificates

ENV RUST_BACKTRACE=1
ENV RUST_LOG="debug,rust_server=debug,actix_web=debug,sqlx=debug"

ENTRYPOINT ["/app/rust_server"]
