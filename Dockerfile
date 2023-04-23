####################################################################################################
## Builder
####################################################################################################
FROM quay.io/fedora/fedora:38 AS builder
RUN dnf upgrade -y && dnf install -y rust cargo

WORKDIR /app

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM quay.io/fedora/fedora-minimal:38

WORKDIR /app

COPY --from=builder /app/target/release/actix-qrcode ./

# Use an unprivileged user.
USER 1001

ENV BIND_ADDRESS="0.0.0.0"

EXPOSE 8080

CMD ["/app/actix-qrcode"]