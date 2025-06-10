FROM rust:1.87 AS build

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:12-slim

COPY --from=build /app/target/release/mmm-backend /usr/local/bin/mmm-backend 

ENTRYPOINT ["mmm-backend"]
