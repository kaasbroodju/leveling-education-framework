# Multi-stage build for optimal image size
FROM rust:1.89 AS builder

WORKDIR /app

# Copy manifests and Rocket config
COPY Cargo.toml Cargo.lock Rocket.toml ./

# Copy source code
COPY src ./src
COPY app ./app

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

## Install CA certificates for HTTPS requests (if needed)
#RUN apt-get update && apt-get install -y \
#    ca-certificates \
#    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy the binary and Rocket config
COPY --from=builder /app/target/release/leveling-education-framework /usr/local/bin/app
COPY --from=builder /app/Rocket.toml /app/Rocket.toml

# Set working directory for runtime
WORKDIR /app

# Set ownership
RUN chown -R appuser:appuser /app /usr/local/bin/app

# Switch to non-root user
USER appuser

# Expose port 3000 (configured in Rocket.toml)
EXPOSE 3000

# Environment for logging only
ENV RUST_LOG=info

# Run the binary (Rocket.toml will be found automatically)
CMD ["/usr/local/bin/app"]