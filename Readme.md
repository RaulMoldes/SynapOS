# SynapOS

This project is inspired by: https://os.phil-opp.com/ and https://tockos.org/

My goal is to create a Minimalistic Operating System for edge devices to run inference on bare metal.


To compile this project, add your specific device's target triple (arch-vendor-runtime) as a target to help cargo build it.

Example: thumbv7em-none-eabihf

```bash
rustup target add thumbv7em-none-eabihf
```
And then you can:
```bash
cargo build --target thumbv7em-none-eabihf
```

Aternatively, you can compile for your host system:

```bash
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```
Run on QEMU
```bash
sudo qemu-system-x86_64 -drive format=raw,file=/home/raulmoldes60nd/Desktop/proyectos/SynapOS/target/x86_64_synap/debug/bootimage-synapos.bin
```