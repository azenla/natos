#!/bin/sh
set -e

ALL_COMMANDS="kitty purr"

cd "$(dirname "${0}")/.." || exit 1

. "hack/common.sh"

echo "[initrd build] ${TARGET_ARCH} ${RUST_PROFILE}"
mkdir -p "${FINAL_DIR}"

cargo build --bin init --target "${RUST_TARGET}" --profile "${RUST_PROFILE}"

for CMD in ${ALL_COMMANDS}; do
	cargo build --bin "${CMD}" --target "${RUST_TARGET}" --profile "${RUST_PROFILE}"
done

rm -rf "target/initrd-${TARGET_ARCH}"
mkdir -p "target/initrd-${TARGET_ARCH}/work"
install -Dm755 "target/${RUST_TARGET}/${RUST_TARGET_SUBDIR}/init" "target/initrd-${TARGET_ARCH}/work/init"
mkdir "target/initrd-${TARGET_ARCH}/work/bin"

for CMD in ${ALL_COMMANDS}; do
	install -Dm755 "target/${RUST_TARGET}/${RUST_TARGET_SUBDIR}/${CMD}" "target/initrd-${TARGET_ARCH}/work/bin/${CMD}"
done

for ITEM in ${INITRD_INCLUDE_BINARIES}
do
  CMD="$(basename "${ITEM}")"
  install -Dm755 "${ITEM}" "target/initrd-${TARGET_ARCH}/work/bin/${CMD}"
done

cd "target/initrd-${TARGET_ARCH}/work" || exit 1
if [ "$(uname)" = "Darwin" ]; then
	find . | cpio -R 0:0 -o -H newc --quiet >"../initrd"
else
	find . | cpio -R 0:0 --ignore-devno --renumber-inodes -o -H newc --quiet >"../initrd"
fi

cd ../../.. || exit 1
cd "$(dirname "${0}")/.." || exit 1

if [ "${DOCKER_BUILD}" = "1" ]
then
  cp "target/initrd-${TARGET_ARCH}/initrd" "/initrd"
fi
