# builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1.97.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# build deps, not the project
RUN cargo chef cook --release --recipe-path recipe.json
# up to this point, if our dependency tree stays the same,
# all layers should be cached.
ENV SQLX_OFFLINE=true
# build project
RUN cargo build --release --bin zero2prod

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
