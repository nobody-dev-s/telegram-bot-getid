ARG RUST_VERSION=1.72.1
ARG APP_NAME=telegram-bot-getid
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

RUN apt-get update 
RUN apt-get install -y pkg-config
RUN apt-get install -y libssl-dev

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/getid
EOF

FROM debian:bullseye-slim AS final

ARG UID=10001
RUN apt-get update
RUN apt-get install -y ca-certificates
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/getid /bin/

CMD ["/bin/getid"]
