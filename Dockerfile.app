FROM alpine:latest

COPY ./target/release/rust_server /app/app

ENV RUST_BACKTRACE=1
ENV RUST_LOG="info,actix_web=debug,sqlx=error"

ENTRYPOINT ["/app/app"]
