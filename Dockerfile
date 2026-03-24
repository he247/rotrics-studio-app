# Use the official Rust image as the base
FROM rust:1.75-slim-bookworm as builder

# Install system dependencies for Tauri and SerialPort
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config \
    udev \
    libudev-dev

# Set the working directory
WORKDIR /usr/src/app

# Copy the Cargo files first to cache dependencies
COPY Cargo.toml ./
# Create a dummy src/main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Copy the actual source code
COPY . .

# Build the application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-37 \
    libgtk-3-0 \
    libudev1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/rotrics-studio-app .

CMD ["./rotrics-studio-app"]
