const VGA: *mut u8 = 0xb8000 as *mut u8;

pub fn write(s: &str) {
	let mut i = 0;
	for b in s.bytes() {
		unsafe {
			*VGA.add(i) = b;
			*VGA.add(i + 1) = 0x0F;
		}
		i += 2;
	}
}