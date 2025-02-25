# Use an official Rust image as base
FROM rust:1.85  

# Set the working directory inside the container
WORKDIR /app

# Update Cargo to the latest version
RUN rustup update stable && cargo --version

# Copy source files
COPY . .

# Build the Rust program
RUN cargo build --release

# Set the command to run the compiled binary
CMD ["./target/release/fibbot"]




#hello 
