# Use a Rust base image
FROM rust:latest

# Create a new directory for the server
RUN mkdir /app
WORKDIR /app

# Copy your Rust server code and any required files into the container
COPY src .

# Build the server
RUN cargo build --release

# Set the server as the entrypoint when the container starts
CMD ["./target/release/build"]
