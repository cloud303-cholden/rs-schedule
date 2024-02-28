FROM rust:1.76-slim-bullseye as build

# Create empty project
RUN USER=root cargo new --bin schedule
WORKDIR /schedule

# Copy manifests
COPY Cargo.toml .

# Cache dependencies
RUN cargo build --release

# Copy source
COPY src .

# Build for release
RUN rm target/release/deps/schedule*
RUN cargo build --release

# Create fresh environment
FROM debian:bullseye-slim

# Copy build artifacts
COPY --from=build /schedule/target/release/schedule .

CMD ["./schedule"]
