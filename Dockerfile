FROM rust:1.79.0 AS builder
WORKDIR /grandma_drink_water_notification
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release
COPY ./src ./src
RUN rm -f target/release/deps/grandma_drink_water_notification*
RUN cargo build --release

FROM debian:latest

COPY --from=builder /grandma_drink_water_notification/target/release/grandma_drink_water_notification /usr/local/bin/grandma_drink_water_notification
CMD [ "grandma_drink_water_notification" ]