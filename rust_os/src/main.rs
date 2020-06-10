#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod vga_driver;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! 
{
    use core::fmt::Write;
    
    vga_driver::WRITER.lock().write_string("test");
    write!(vga_driver::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}