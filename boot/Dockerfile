FROM --platform=$BUILDPLATFORM debian:trixie@sha256:b7e663fabda920f28abf26e7fbfe0867145645fdf3e47a42323147242151e066 AS build
ARG BUILDPLATFORM
ARG EFI_NAME
RUN export DEBIAN_FRONTEND=noninteractive && apt-get update && apt-get install -y \
      parted dosfstools mtools && \
      rm -rf /var/lib/apt/lists/*
WORKDIR /work
COPY natos.efi /work/${EFI_NAME}.EFI
RUN truncate -s256MiB natos.img && \
    parted --script natos.img mklabel gpt > /dev/null 2>&1 && \
    parted --script natos.img mkpart primary fat32 1MiB 100% > /dev/null 2>&1 && \
    parted --script natos.img set 1 esp on > /dev/null 2>&1 && \
    mkfs.vfat -F32 -n EFI natos.img && \
    mmd -i natos.img ::/EFI && \
    mmd -i natos.img ::/EFI/BOOT && \
    mcopy -i natos.img ${EFI_NAME}.EFI ::/EFI/BOOT/ && \
    mv natos.img /natos.img
RUN truncate -s256MiB mini.img && \
    parted --script mini.img mklabel gpt > /dev/null 2>&1 && \
    parted --script mini.img mkpart primary fat32 1MiB 100% > /dev/null 2>&1 && \
    parted --script mini.img set 1 esp on > /dev/null 2>&1 && \
    mkfs.vfat -F32 -n EFI mini.img && \
    mmd -i mini.img ::/EFI && \
    mmd -i mini.img ::/EFI/BOOT && \
    mcopy -i mini.img ${EFI_NAME}.EFI ::/EFI/BOOT/ && \
    mv mini.img /mini.img

FROM scratch AS final
COPY --from=build /natos.img /natos.img
COPY --from=build /mini.img /mini.img
