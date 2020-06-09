#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod vga_driver;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_driver::print_vga(b"write to vga!");
    loop {}
}