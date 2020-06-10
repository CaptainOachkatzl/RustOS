#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod vga_driver;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! 
{
    println!("Hello world{}", 123);

    loop {}
}