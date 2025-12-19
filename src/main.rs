#![no_std]
#![no_main]

mod multiboot;
mod arch;

use core::panic::PanicInfo;
use arch::x86::drivers::framebuffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
	framebuffer::write("Hello from Rust kernel");
	loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
	loop {}
}