# syntax=docker/dockerfile:1.7-labs
ARG RUST_PROFILE=release

FROM rust:1.88.0-alpine@sha256:9dfaae478ecd298b6b5a039e1f2cc4fc040fc818a2de9aa78fa714dea036574d AS build
RUN apk --no-cache add musl-dev busybox-static
ARG RUST_PROFILE
RUN adduser -S -s /bin/sh build
COPY \
    --exclude=kernel --exclude=uki --exclude=vm \
    --exclude=boot --exclude=mini --exclude=rust-toolchain.toml \
    --chown=build:build \
    . /build
WORKDIR /build
ENV DOCKER_BUILD=1
RUN ./initrd/build.sh "native" "${RUST_PROFILE}"

FROM scratch AS final
COPY --from=build /initrd /initrd
