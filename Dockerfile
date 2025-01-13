FROM lukemathwalker/cargo-chef:latest-rust-1.83.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE=true
# Build our project
RUN cargo build --release --bin zero2prod

FROM cgr.dev/chainguard/glibc-dynamic
COPY --from=build --chown=nonroot:nonroot /app/target/release/zero2prod /usr/local/bin/zero2prod
COPY --chown=nonroot:nonroot configuration /usr/local/bin/configuration
ENV APP_ENVIRONMENT=production
CMD ["/usr/local/bin/zero2prod"]