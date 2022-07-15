FROM rust:1.62.0-slim as builder
ADD src/ /source/src
ADD Cargo.toml Cargo.lock /source/
WORKDIR /source
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /source/target/release/axum-transform /axum-transform
RUN chmod 777 /axum-transform
WORKDIR /
ENV AXUM_hostname 0.0.0.0
ENTRYPOINT ["./axum-transform"]