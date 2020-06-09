pub fn print_vga(text: &[u8]) {

    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in text.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
