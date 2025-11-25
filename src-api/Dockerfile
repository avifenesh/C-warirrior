# Build with musl for static binary
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY . .

# Build statically linked binary
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release -p code-warrior-api

# Runtime - Alpine for small image
FROM alpine:latest

RUN apk add --no-cache gcc libc-dev ca-certificates

COPY --from=builder /app/target/release/code-warrior-api /usr/local/bin/

ENV RUST_LOG=info
EXPOSE 3000

CMD ["code-warrior-api"]
