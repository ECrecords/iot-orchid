FROM clux/muslrust:stable AS builder

# Set up environment to ensure SQLx can operate in offline mode
ENV SQLX_OFFLINE=true

WORKDIR /usr/src/iot-orchid

# Copy your source code into the Docker image
COPY . .

# Build the project
# The muslrust image is configured for static builds, so just run cargo build
RUN cargo build --release

# Use a minimal runtime image
# Since we're building a fully static binary, we can use FROM scratch
FROM scratch

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/iot-orchid/target/x86_64-unknown-linux-musl/release/iot-orchid /iot-orchid

# Set the binary as the entrypoint
ENTRYPOINT ["/iot-orchid"]

# Expose the port the application listens on
EXPOSE 8000
