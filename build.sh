#!/usr/bin/env bash

set -ex

cargo build
rm -fr isodir

mkdir -p isodir/boot/grub
cp target/target/debug/test-os isodir/boot/myos.bin
cp grub.cfg isodir/boot/grub/grub.cfg
grub-mkrescue -o myos.iso isodir
