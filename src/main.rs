#![no_std]
#![no_main]

mod multiboot;

use core::panic::PanicInfo;

const VGA: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	write("Hello from Rust kernel");
	loop {}
}

fn write(s: &str) {
	let mut i = 0;
	for b in s.bytes() {
		unsafe {
			*VGA.add(i) = b;
			*VGA.add(i + 1) = 0x0F;
		}
		i += 2;
	}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
	loop {}
}