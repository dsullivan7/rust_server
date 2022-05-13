FROM alpine:latest

COPY ./target/release/rust_server /app/app

ENTRYPOINT ["/app/app"]
