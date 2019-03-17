# Step 1: build the binary in release mode using musl
FROM clux/muslrust AS build

ARG EXAMPLE=hello_world

RUN mkdir -p /src
WORKDIR /src
COPY . /src

RUN cargo build --release --example $EXAMPLE
RUN strip target/x86_64-unknown-linux-musl/release/examples/$EXAMPLE
RUN cp target/x86_64-unknown-linux-musl/release/examples/$EXAMPLE main

# Step 2: retrieve SSL certificates
FROM alpine as certs

RUN apk update && apk add ca-certificates

# Step 3: create final image with the binary at the expected places
# and the SSL certificates
FROM busybox:musl

COPY --from=certs /etc/ssl/certs /etc/ssl/certs

COPY --from=build /src/main /opt/resource/main
RUN ln -s /opt/resource/main /opt/resource/check
RUN ln -s /opt/resource/main /opt/resource/in
RUN ln -s /opt/resource/main /opt/resource/out

ENV SSL_CERT_FILE /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR /etc/ssl/certs
