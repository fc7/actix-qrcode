# https://kerkour.com/rust-small-docker-image/
####################################################################################################
## Builder
####################################################################################################
FROM registry.access.redhat.com/ubi9:9.1 AS builder

RUN dnf upgrade && dnf install -y rust-toolset

WORKDIR /app

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM registry.access.redhat.com/ubi9-minimal:9.1

WORKDIR /app

COPY --from=builder /app/target/release/actix-qrcode ./

# Use an unprivileged user.
USER 1001

ENV BIND_ADDRESS="0.0.0.0"

CMD ["/app/actix-qrcode"]