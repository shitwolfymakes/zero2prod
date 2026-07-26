# builder stage
FROM rust:1.97.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt update -y \
    && apt install -y --no-install-recommends openssl ca-certificates \
    && apt autoremove -y \
    && apt clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENV=production
ENTRYPOINT [ "./zero2prod" ]
