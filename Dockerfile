FROM rust:1-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/swadedsbot /usr/local/bin/swadedsbot
RUN useradd --system --home /var/lib/swadedsbot --create-home swadedsbot
USER swadedsbot
WORKDIR /var/lib/swadedsbot
ENTRYPOINT ["/usr/local/bin/swadedsbot"]
