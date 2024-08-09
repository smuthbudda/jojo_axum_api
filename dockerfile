# Use the official Rust image as a build environment
FROM rust:latest as builder

# Create and set the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src directory to ensure dependencies are cached
RUN mkdir src && echo 'fn main() { println!("If you see this, something went wrong!"); }' > src/main.rs

# Build dependencies
RUN cargo build --release

# Remove the dummy src/main.rs
RUN rm src/main.rs

# Copy the actual source code
COPY . .

# Build the actual application
RUN cargo build --release

# Use a minimal base image for the final runtime environment
FROM debian:stable-slim

# Set the working directory
WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust_api_axum_diesel .

COPY .env .env


# Set the entrypoint command
CMD ["./rust_api_axum_diesel"]

# Expose the port that the application will run on
EXPOSE 8080
