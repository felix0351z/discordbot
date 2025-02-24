
# Builder
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /DiscordBot
COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

# Final image
FROM scratch
COPY --from=builder /DiscordBot/target/x86_64-unknown-linux-musl/release/DiscordBot ./
COPY --from=builder /DiscordBot/settings.toml ./
ENTRYPOINT ["/DiscordBot"]