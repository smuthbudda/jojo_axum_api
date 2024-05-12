# Use the official Rust image as the base
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to get dependencies cached
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy source file
RUN rm src/main.rs

# Copy the actual source code
COPY src ./src

# Build the release binary
RUN cargo install --path .

# Expose the port your Axum server listens on
EXPOSE 8000

# Set the entry point to run the binary
CMD ["jojo_axum_api"]
