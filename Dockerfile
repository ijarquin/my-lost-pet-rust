# Use the official Rust image as a base.
# Using a specific version is good practice for reproducibility.
FROM rust:1.87

# Install system dependencies that might be needed for building crates.
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Install dioxus-cli, which is used to serve the application.
RUN cargo install dioxus-cli

# Add the wasm target for Rust, required for web platform builds.
RUN rustup target add wasm32-unknown-unknown

# Set the working directory inside the container.
WORKDIR /usr/src/app

# Expose the default ports for the Dioxus development server and hot-reloading.
EXPOSE 8080
EXPOSE 3001

# The default command to run the application. This will be used by docker-compose.
CMD ["dx", "serve", "--port", "8080"]

