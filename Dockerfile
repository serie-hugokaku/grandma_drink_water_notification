FROM rust:1.79.0
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
ENV PORT 8080
ENTRYPOINT ["target/release/grandma_drink_water_notification"]