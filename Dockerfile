FROM debian:bookworm-slim

WORKDIR /app

COPY rust-lab /app/rust-lab
COPY static /app/static

EXPOSE 3000

CMD ["./rust-lab"]

