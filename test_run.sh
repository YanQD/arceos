ARCH=riscv64
APP=loader

make A=examples/${APP} ARCH=${ARCH} run LOG=trace QEMU_LOG=y
