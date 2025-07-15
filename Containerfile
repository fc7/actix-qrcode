####################################################################################################
## Builder
####################################################################################################
FROM registry.access.redhat.com/ubi9/ubi:9.6 AS builder
RUN dnf upgrade -y && dnf install -y rust cargo

WORKDIR /tmp

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM registry.access.redhat.com/ubi9/ubi-micro:9.6

WORKDIR /app

COPY --from=builder /tmp/target/release/actix-qrcode ./

# Use an unprivileged user.
USER 1001

ENV BIND_ADDRESS="0.0.0.0"

EXPOSE 8080

CMD ["./actix-qrcode"]