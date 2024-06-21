#!/usr/bin/env bash

qemu-system-i386 -serial stdio -device isa-debug-exit,iobase=0xf4,iosize=0x04 -kernel target/target/debug/rust-kernel

exit $(($? >> 1))
