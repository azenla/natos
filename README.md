# natOS

natOS, a simple operating system.

## Table of Contents

- [natOS](#natos)
    * [Components](#components)
        + [kernel](#kernel)
        + [init](#init)
        + [kitty](#kitty)
    * [Build Guide](#build-guide)
        + [Build Requirements](#build-requirements)
    * [Boot Guide](#boot-guide)
        + [Boot Requirements](#boot-requirements)

## Components

### kernel

Linux kernel default build, bootable by QEMU and basic systems.

### init

A basic init that sets up the system and calls kitty.

### kitty

The interactive shell of natOS.

## Build Guide

### Build Requirements

- [docker](https://docs.docker.com/engine/install/): Required to build the kernel and initrd.

```bash
# Build an x86_64 release build.
# Produces a kernel and initrd in target/final/x86_64
./hack/build.sh x86_64 release
```

```bash
# Build an x86_64 debug build.
# Produces a kernel and initrd in target/final/x86_64
./hack/build.sh x86_64 debug
```

```bash
# Build an aarch64 release build.
# Produces a kernel and initrd in target/final/aarch64
./hack/build.sh aarch64 release
```

```bash
# Build an aarch64 debug build.
# Produces a kernel and initrd in target/final/aarch64
./hack/build.sh aarch64 debug
```

## Boot Guide

### Boot Requirements

- [docker](https://docs.docker.com/engine/install/): Required to build the kernel and initrd.
- [qemu](https://www.qemu.org/): Required for running guests.

```bash
# Boot an x86_64 release build.
./hack/boot.sh x86_64 release
```

```bash
# Boot an x86_64 debug build.
./hack/boot.sh x86_64 debug
```

```bash
# Boot an aarch64 release build.
./hack/boot.sh aarch64 release
```

```bash
# Boot an aarch64 debug build.
./hack/boot.sh aarch64 debug
```
