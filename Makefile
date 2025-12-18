release:
	cargo build --release
	cp target/x86_64-unknown-none/release/rust-kernel iso/boot/kernel.elf
	grub-mkrescue -o kernel.iso iso

run: release
	qemu-system-x86_64 -cdrom kernel.iso

clean:
	cargo clean
	rm -f kernel.iso

.PHONY: release run clean