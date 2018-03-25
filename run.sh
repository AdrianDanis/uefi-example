#!/bin/sh

set -e

TEMP=$(mktemp --tmpdir=$PWD -d)

mkdir -p $TEMP

dd if=/dev/zero of=$TEMP/uefi.img bs=512 count=93750
parted $TEMP/uefi.img -s -a minimal mklabel gpt
parted $TEMP/uefi.img -s -a minimal mkpart EFI FAT16 2048s 93716s
parted $TEMP/uefi.img -s -a minimal toggle 1 boot
dd if=/dev/zero of=$TEMP/temp_part.img bs=512 count=91669
mformat -i $TEMP/temp_part.img -h 32 -t 32 -n 64 -c 1

mmd -i $TEMP/temp_part.img ::/EFI
mmd -i $TEMP/temp_part.img ::/EFI/BOOT
cp $1 $TEMP/BOOTX64.EFI
mcopy -i $TEMP/temp_part.img $TEMP/BOOTX64.EFI ::/EFI/BOOT
dd if=$TEMP/temp_part.img of=$TEMP/uefi.img bs=512 count=91669 seek=2048 conv=notrunc

cp OVMF.fd.original $TEMP/OVMF.fd

qemu-system-x86_64 -M pc -m 64 -nographic -drive if=pflash,format=raw,file=$TEMP/OVMF.fd -drive id=disk0,if=none,format=raw,file=$TEMP/uefi.img \
    -device virtio-blk-pci,drive=disk0,bootindex=0 -global PIIX4_PM.disable_s3=0 -serial mon:stdio -device piix3-usb-uhci -device usb-tablet \
    -netdev id=net0,type=user -device virtio-net-pci,netdev=net0,romfile= -device qxl-vga

rm -rf $TEMP
