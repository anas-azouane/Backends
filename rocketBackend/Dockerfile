FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
RUN apt-get update && apt-get install -y musl-tools pkg-config libpq-dev

COPY ../static .
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

# Final image
FROM alpine:latest
RUN apk add --no-cache ca-certificates libpq

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rocketBackend .
COPY ./static /static
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
EXPOSE 8000
CMD ["./rocketBackend"]

