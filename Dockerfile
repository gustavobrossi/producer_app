# Stage 1: Build the application
FROM rust:1.72 AS builder

# Set the working directory
WORKDIR /producer_app

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final image
FROM scratch

# Copy the compiled binary from the builder stage
COPY --from=builder /producer_app/target/release/producer_app /usr/local/bin/producer_app

# Copy the necessary dependencies from the builder stage
COPY --from=builder /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1
COPY --from=builder /lib/x86_64-linux-gnu/libgcc_s.so.1 /lib/x86_64-linux-gnu/libgcc_s.so.1
COPY --from=builder /lib/x86_64-linux-gnu/libm.so.6 /lib/x86_64-linux-gnu/libm.so.6
COPY --from=builder /lib/x86_64-linux-gnu/libc.so.6 /lib/x86_64-linux-gnu/libc.so.6
COPY --from=builder /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2

# Set the entrypoint to the binary
ENTRYPOINT ["/usr/local/bin/producer_app"]