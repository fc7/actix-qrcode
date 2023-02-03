# https://kerkour.com/rust-small-docker-image/
####################################################################################################
## Builder
####################################################################################################
FROM docker.io/rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=actix
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:bullseye-slim
RUN apt-get upgrade && rm -rf /var/lib/apt/lists/*

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/actix-qrcode ./

# Use an unprivileged user.
USER actix:actix

ENV BIND_ADDRESS="0.0.0.0"

CMD ["/app/actix-qrcode"]