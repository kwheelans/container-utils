FROM lukemathwalker/cargo-chef:latest AS chef

FROM chef AS planner
WORKDIR /recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app

# Build dependencies
COPY --from=planner /recipe/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --frozen

# Final image
FROM debian:13-slim
WORKDIR /app
ENV PATH=/app:$PATH
COPY --from=builder /app/target/release/container-utils /app
