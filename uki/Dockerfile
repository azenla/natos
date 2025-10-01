FROM debian:trixie@sha256:b7e663fabda920f28abf26e7fbfe0867145645fdf3e47a42323147242151e066 AS build
RUN export DEBIAN_FRONTEND=noninteractive && apt-get update && apt-get install -y \
      systemd-ukify && \
      rm -rf /var/lib/apt/lists/*
COPY kernel /kernel
COPY initrd /initrd
COPY cmdline /cmdline
RUN ukify build --output=/natos.efi --linux=/kernel --initrd=/initrd --cmdline=@/cmdline

FROM scratch AS final
COPY --from=build /natos.efi /natos.efi
