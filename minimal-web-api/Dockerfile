FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

ENV APP=minimal-web-api
ENV USER=minimal-web-api
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /${APP}

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /minimal-web-api

# Copy our build
COPY --from=builder /minimal-web-api/target/x86_64-unknown-linux-musl/release/minimal-web-api ./

# Use an unprivileged user.
USER minimal-web-api:minimal-web-api

EXPOSE 3030
CMD ["/minimal-web-api/minimal-web-api"]