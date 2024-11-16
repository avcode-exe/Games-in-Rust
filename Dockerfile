# Use the official Rust image as the base image
FROM rust:1.72

# Set the working directory inside the container
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Install dependencies
RUN apt update && apt install -y libncurses5-dev libncursesw5-dev

# Build the dependencies
RUN cargo fetch
RUN cargo build --release --locked

# Build the project
RUN cargo build --release

# Set the entrypoint to the compiled binary
CMD ["/usr/src/app/target/release/games-in-rust"]