FROM rust:1-slim AS chef 

RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching layer
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin k-qr

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install necessary runtime dependencies (if any, usually none for static rust binaries)
# Create a non-root user
RUN groupadd -g 10001 appgroup && \
    useradd -u 10001 -g appgroup appuser

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/k-qr /usr/local/bin/k-qr

# Set default environment variables
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=3000
ENV RUST_LOG=k_qr=info

# Use the non-root user
USER appuser

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/k-qr"]