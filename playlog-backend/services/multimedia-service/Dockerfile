# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.93.1
ARG APP_NAME=multimedia-service

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git curl

# Build the application.
RUN --mount=type=bind,source=services,target=services \
    --mount=type=bind,source=shared,target=shared \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release -p $APP_NAME && \
cp ./target/release/$APP_NAME /bin/server

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
FROM alpine:3.18 AS final

# Create a non-privileged user that the app will run under.
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Copy JWT key.
COPY --chown=appuser:appuser keys/public.pem /keys/public.pem

# Healthcheck command.
HEALTHCHECK --interval=10s --timeout=5s --retries=10 --start-period=30s \
  CMD wget -q -O /dev/null http://127.0.0.1:3003/api/multimedia-service-health || exit 1

# What the container should run when it is started.
CMD ["/bin/server"]
