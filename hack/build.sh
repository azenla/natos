#!/bin/sh
set -e

cd "$(dirname "${0}")/.." || exit 1

. "hack/common.sh"

EFI_NAME="BOOTX64"
if [ "${TARGET_ARCH}" = "aarch64" ]; then
	EFI_NAME="BOOTAA64"
fi

echo "[build] ${TARGET_ARCH} ${RUST_PROFILE}"

if ! command -v docker >/dev/null 2>&1; then
	echo "ERROR: docker is required to build natOS." >/dev/stderr
	exit 1
fi

export DOCKER_CLI_HINTS="0"

if [ "${SKIP_CLEANUP}" != 1 ]; then
	rm -rf "${FINAL_DIR}"
fi
mkdir -p "${FINAL_DIR}"

if [ "${SKIP_KERNEL_BUILD}" != "1" ] || [ "${SKIP_INITRD_BUILD}" != "1" ] ||
	[ "${SKIP_UKI_BUILD}" != "1" ] || [ "${SKIP_VM_BUILD}" != "1" ] ||
	[ "${SKIP_MINI_BUILD}" != "1" ] || [ "${MINI_EFI_ONLY}" = "1" ]; then
	docker build -t "${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" -f hack/utils/Dockerfile.copy hack
fi

if [ "${SKIP_KERNEL_BUILD}" != "1" ] && [ "${MINI_EFI_ONLY}" != "1" ]; then
	echo "[kernel build] ${TARGET_ARCH} ${RUST_PROFILE}"
	docker build --platform="${DOCKER_TARGET}" -t "${DOCKER_PREFIX}/natos-kernel-${TARGET_ARCH}:${DOCKER_TAG}" -f kernel/Dockerfile kernel

	if [ "${KERNEL_BUILD_TAG}" = "1" ]; then
		docker build --platform="${DOCKER_TARGET}" -t "${DOCKER_PREFIX}/natos-kernel-build-${TARGET_ARCH}:${DOCKER_TAG}" -f kernel/Dockerfile --target build kernel
	fi

	docker run --rm -i \
		--mount="type=image,source=${DOCKER_PREFIX}/natos-kernel-${TARGET_ARCH}:${DOCKER_TAG},target=/image" \
		"${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" cat /image/kernel >"${FINAL_DIR}/kernel"

	if [ "${TARGET_ARCH}" = "x86_64" ]; then
		echo "console=ttyS0 quiet" >"${FINAL_DIR}/cmdline"
	elif [ "${TARGET_ARCH}" = "aarch64" ]; then
		echo "quiet" >"${FINAL_DIR}/cmdline"
	fi
fi

if [ "${SKIP_INITRD_BUILD}" != "1" ] && [ "${MINI_EFI_ONLY}" != "1" ]; then
	echo "[initrd build] ${TARGET_ARCH} ${RUST_PROFILE}"
	if [ "${INITRD_BUILD_LOCAL}" = "1" ]; then
		./initrd/build.sh "${TARGET_ARCH}" "${RUST_PROFILE}"
	else
		docker build --platform="${DOCKER_TARGET}" -t "${DOCKER_PREFIX}/natos-initrd-${TARGET_ARCH}:${DOCKER_TAG}" -f initrd/Dockerfile --build-arg "RUST_PROFILE=${RUST_PROFILE}" .
		docker run --rm -i \
			--mount="type=image,source=${DOCKER_PREFIX}/natos-initrd-${TARGET_ARCH}:${DOCKER_TAG},target=/image" \
			"${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" cat /image/initrd >"${FINAL_DIR}/initrd"
	fi
fi

if [ "${SKIP_UKI_BUILD}" != "1" ] && [ "${MINI_EFI_ONLY}" != "1" ]; then
	echo "[uki build] ${TARGET_ARCH} ${RUST_PROFILE}"
	docker build --platform="${DOCKER_TARGET}" -t "${DOCKER_PREFIX}/natos-uki-${TARGET_ARCH}:${DOCKER_TAG}" -f uki/Dockerfile "${FINAL_DIR}"
	docker run --rm -i \
		--mount="type=image,source=${DOCKER_PREFIX}/natos-uki-${TARGET_ARCH}:${DOCKER_TAG},target=/image" \
		"${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" cat /image/natos.efi >"${FINAL_DIR}/natos.efi"

	mkdir -p "${FINAL_DIR}/efi/EFI/BOOT"
	cp "${FINAL_DIR}/natos.efi" "${FINAL_DIR}/efi/EFI/BOOT/${EFI_NAME}.efi"
fi

if [ "${SKIP_VM_BUILD}" != "1" ]; then
	echo "[vm build] ${TARGET_ARCH} ${RUST_PROFILE}"
	docker build --platform="${DOCKER_TARGET}" -t "${DOCKER_PREFIX}/natos-ovmf-${TARGET_ARCH}:${DOCKER_TAG}" -f vm/Dockerfile.ovmf "${FINAL_DIR}"
	docker run --rm -i \
		--mount="type=image,source=${DOCKER_PREFIX}/natos-ovmf-${TARGET_ARCH}:${DOCKER_TAG},target=/image" \
		"${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" cat /image/ovmf.fd >"${FINAL_DIR}/ovmf.fd"
fi

if [ "${SKIP_MINI_BUILD}" != "1" ]; then
	echo "[mini build] ${TARGET_ARCH} ${RUST_PROFILE}"
	docker build --platform="${DOCKER_TARGET}" -t "${DOCKER_PREFIX}/natos-mini-${TARGET_ARCH}:${DOCKER_TAG}" --build-arg="RUST_TARGET_SUBDIR=${RUST_TARGET_SUBDIR}" -f mini/Dockerfile mini
	docker run --rm -i \
		--mount="type=image,source=${DOCKER_PREFIX}/natos-mini-${TARGET_ARCH}:${DOCKER_TAG},target=/image" \
		"${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" cat /image/mini.efi >"${FINAL_DIR}/mini.efi"
	mkdir -p "${FINAL_DIR}/efi-mini/EFI/BOOT"
	cp "${FINAL_DIR}/mini.efi" "${FINAL_DIR}/efi-mini/EFI/BOOT/${EFI_NAME}.EFI"
fi

if [ "${SKIP_BOOT_BUILD}" != "1" ] && [ "${MINI_EFI_ONLY}" != "1" ]; then
	echo "[boot build] ${TARGET_ARCH} ${RUST_PROFILE}"
	docker build --platform="${DOCKER_TARGET}" -t "${DOCKER_PREFIX}/natos-boot-${TARGET_ARCH}:${DOCKER_TAG}" --build-arg "EFI_NAME=${EFI_NAME}" -f boot/Dockerfile "${FINAL_DIR}"
	docker run --rm -i \
		--mount="type=image,source=${DOCKER_PREFIX}/natos-boot-${TARGET_ARCH}:${DOCKER_TAG},target=/image" \
		"${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" cat /image/natos.img >"${FINAL_DIR}/natos.img"
	docker run --rm -i \
		--mount="type=image,source=${DOCKER_PREFIX}/natos-boot-${TARGET_ARCH}:${DOCKER_TAG},target=/image" \
		"${DOCKER_PREFIX}/natos-utils-copy:${DOCKER_TAG}" cat /image/mini.img >"${FINAL_DIR}/mini.img"
fi
