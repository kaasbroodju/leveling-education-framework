# Multi-stage build for optimal image size
FROM rust:1.82 AS builder

WORKDIR /app

# Copy manifests and Rocket config
COPY Cargo.toml Cargo.lock Rocket.toml ./

# Copy source code and required folders
COPY src ./src
COPY app/data ./app/data
COPY app/public ./app/public

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime


# Create app user
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copy the binary, Rocket config, and data folders
COPY --from=builder /app/target/release/leveling-education-framework /app/app
COPY --from=builder /app/Rocket.toml /app/Rocket.toml
COPY --from=builder /app/app/data /app/app/data
COPY --from=builder /app/app/public /app/app/public

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
CMD ["./app"]