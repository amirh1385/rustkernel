#[repr(C, align(8))]
pub struct Multiboot2Header {
	data: [u32; 8],
}

#[used]
#[link_section = ".multiboot"]
pub static MULTIBOOT2_HEADER: Multiboot2Header = Multiboot2Header {
	data : [
		0xE85250D6,
		0,
		32,
		!(0xE85250D6u32 + 0 + 32) + 1,

		0,
		0,
		8,
		0,
	],
};