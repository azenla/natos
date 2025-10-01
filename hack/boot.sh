#!/bin/sh
set -e

cd "$(dirname "${0}")/.." || exit 1

. "hack/common.sh"

./hack/build.sh "${TARGET_ARCH}" "${RUST_PROFILE}"

clear

set --

if [ "${TARGET_ARCH}" = "x86_64" ]; then
	set -- "${@}" qemu-system-x86_64 -M q35
elif [ "${TARGET_ARCH}" = "aarch64" ]; then
	set -- "${@}" qemu-system-aarch64 -M virt -cpu cortex-a57
fi

set -- "${@}" -smp 2 -m 4096

if [ "${NO_GRAPHICAL_BOOT}" = "1" ]; then
	set -- "${@}" -nographic
else
	set -- "${@}" -serial stdio -vga none -device "virtio-gpu,edid=on,xres=1440,yres=900"
fi

if [ "${DIRECT_BOOT}" = "1" ] && [ "${MINI_BOOT}" != "1" ]; then
	if [ -n "${APPEND_CMDLINE}" ]; then
		APPEND_CMDLINE=" ${APPEND_CMDLINE}"
	fi

	set -- "${@}" \
		-kernel "${FINAL_DIR}/kernel" \
		-initrd "${FINAL_DIR}/initrd" \
		-append "$(cat "${FINAL_DIR}/cmdline")${APPEND_CMDLINE}"
else
	rm -f "${FINAL_DIR}/ovmf-boot.fd"
	cp "${FINAL_DIR}/ovmf.fd" "${FINAL_DIR}/ovmf-boot.fd"
	if [ "${TARGET_ARCH}" = "aarch64" ]; then
		dd if=/dev/zero of="${FINAL_DIR}/ovmf-boot.fd" bs=1 count=1 seek=67108863 >/dev/null 2>&1
	fi
	# shellcheck disable=SC2086
	set -- "${@}" \
		-drive "if=pflash,file=${FINAL_DIR}/ovmf-boot.fd,format=raw,readonly=on" \
		-device nvme,drive=disk1,serial=cafebabe

	if [ "${DISK_BOOT}" = "1" ]; then
		if [ "${MINI_BOOT}" = "1" ]; then
			set -- "${@}" \
				-drive "if=none,file=${FINAL_DIR}/mini.img,format=raw,id=disk1,readonly=on"
		else
			set -- "${@}" \
				-drive "if=none,file=${FINAL_DIR}/natos.img,format=raw,id=disk1,readonly=on"
		fi
	else
		if [ "${MINI_BOOT}" = "1" ]; then
			set -- "${@}" \
				-drive "if=none,file=fat:rw:${FINAL_DIR}/efi-mini,format=raw,id=disk1"
		else
			set -- "${@}" \
				-drive "if=none,file=fat:rw:${FINAL_DIR}/efi,format=raw,id=disk1"
		fi
	fi
fi

set -- "${@}" -name "natOS ${TARGET_ARCH}"

exec "${@}"
