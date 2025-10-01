# syntax=docker/dockerfile:1.7-labs
ARG RUST_PROFILE=release
ARG RUST_TARGET_SUBDIR=release

FROM --platform=$BUILDPLATFORM rustlang/rust:nightly-alpine@sha256:2df9949b4b35966397b30d940ebfd63ad2ad62ab5178f84b25b2db6aef830a60 AS build
RUN apk --no-cache add musl-dev busybox-static
ARG RUST_PROFILE
RUN adduser -S -s /bin/sh build
COPY \
    --exclude=rust-toolchain.toml \
    --chown=build:build \
    . /build
WORKDIR /build
ARG TARGETPLATFORM
ARG RUST_TARGET_SUBDIR
RUN if [ "${TARGETPLATFORM}" = "linux/amd64" ] || [ "${TARGETPLATFORM}" = "linux/x86_64" ]; then \
      rustup target add x86_64-unknown-uefi; cargo build --bin mini --profile "${RUST_PROFILE}" --target x86_64-unknown-uefi && \
      cp "target/x86_64-unknown-uefi/${RUST_TARGET_SUBDIR}/mini.efi" /mini.efi; fi
RUN if [ "${TARGETPLATFORM}" = "linux/arm64" ] || [ "${TARGETPLATFORM}" = "linux/aarch64" ]; then \
      rustup target add aarch64-unknown-uefi; cargo build --bin mini --profile "${RUST_PROFILE}" --target aarch64-unknown-uefi && \
      cp "target/aarch64-unknown-uefi/${RUST_TARGET_SUBDIR}/mini.efi" /mini.efi; fi

FROM scratch AS final
COPY --from=build /mini.efi /mini.efi
