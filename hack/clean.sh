#!/bin/sh
set -e

cd "$(dirname "${0}")/.." || exit 1

. "hack/common.sh"

delete_image() {
	IMAGE="${1}"
	docker image ls -q --no-trunc --filter "reference=${DOCKER_PREFIX}/${IMAGE}" | xargs -rn1 docker image rm
}

cargo clean || true

if command -v docker >/dev/null 2>&1; then
	delete_image natos-kernel || true
	delete_image natos-initrd || true
fi
