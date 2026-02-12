FROM debian:bookworm-slim

WORKDIR /app

# Copy prebuilt binary and static files (CI builds wasm and binary beforehand)
COPY rust-lab /app/rust-lab
COPY static /app/static

EXPOSE 3000

CMD ["./rust-lab"]
