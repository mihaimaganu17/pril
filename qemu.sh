qemu-system-x86_64 \
    -m 128 \
    -nographic \
    -bios bios/OVMF.fd \
    -device driver=e1000,netdev=n0 \
    -netdev user,id=n0,tftp=target/x86_64-unknown-uefi/debug,bootfile=pril.efi