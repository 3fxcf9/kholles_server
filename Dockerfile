FROM docker.io/rust:1-slim-bookworm AS build

ARG pkg=kholles_server

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

################################################################################

FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
      git \
      ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && git config --global --add safe.directory /app/content

WORKDIR /app

COPY --from=build /build/main ./

COPY --from=build /build/Rocket.toml .
COPY --from=build /build/static ./static
COPY --from=build /build/templates ./templates

CMD ./main
