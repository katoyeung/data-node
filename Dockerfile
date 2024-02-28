# Use the official Rust image as the base image
FROM rust:latest as builder

# Create a new directory for the application
WORKDIR /usr/src/data_node

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
# This is optional but can help speed up builds if your dependencies don't change
COPY Cargo.toml Cargo.lock ./

# Create a dummy project and build the project's dependencies
# This step is to cache the dependencies and speed up subsequent builds
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/data_node*

# Copy the rest of your application source code
COPY . .

# Build your application with the release profile
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/data-node"]
