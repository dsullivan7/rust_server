FROM alpine:latest

COPY ./target /app/target

ENV RUST_BACKTRACE=1
ENV RUST_LOG="debug,rust_server=debug,actix_web=debug,sqlx=debug"

ENTRYPOINT ["/app/target/debug/rust_server"]
