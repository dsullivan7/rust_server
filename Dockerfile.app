FROM alpine:latest

COPY ./target /app/target

ENV RUST_BACKTRACE=1
ENV RUST_LOG="debug,actix_web=debug,sqlx=debug"

ENTRYPOINT ["/app/target/release/rust_server"]
