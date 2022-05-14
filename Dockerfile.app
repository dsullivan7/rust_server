FROM alpine:latest

COPY ./target/debug/rust_server /app/app

ENV RUST_BACKTRACE=1
ENV RUST_LOG="debug,actix_web=debug,sqlx=debug"

ENTRYPOINT ["/app/app"]
