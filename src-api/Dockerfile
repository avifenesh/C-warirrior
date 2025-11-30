# Build with musl for static binary
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /app
COPY . .

# Build with limited parallelism to avoid OOM
ENV CARGO_BUILD_JOBS=2
RUN RUSTFLAGS='-C target-feature=-crt-static' cargo build --release -p code-warrior-api

# Runtime - Alpine for small image
FROM alpine:latest

# Install GCC for C compilation and nsjail dependencies
RUN apk add --no-cache gcc musl-dev libgcc libstdc++ ca-certificates \
    protobuf libnl3 libcap

# Build nsjail from source (not in Alpine repos)
RUN apk add --no-cache --virtual .build-deps git make g++ protobuf-dev libnl3-dev libcap-dev linux-headers && \
    cd /tmp && \
    git clone --depth 1 https://github.com/google/nsjail.git && \
    cd nsjail && \
    make && \
    mv nsjail /usr/local/bin/ && \
    cd / && \
    rm -rf /tmp/nsjail && \
    apk del .build-deps

COPY --from=builder /app/target/release/code-warrior-api /usr/local/bin/

# Copy nsjail configuration
COPY src-api/nsjail.cfg /app/nsjail.cfg

ENV RUST_LOG=info
EXPOSE 3000

CMD ["code-warrior-api"]
