#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(_boot_info: &'static BootInfo) -> ! 
{
    println!("Starting up OS...");

    println!("Started.");

    rust_os::init();

    println!("It did not crash!");

    #[cfg(test)]
    test_main();
    #[cfg(test)]
    rust_os::exit_qemu(rust_os::QemuExitCode::Success);

    rust_os::hlt_loop();
}

/// This function is called on panic
#[cfg(not(test))] 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    println!("{}", info);
    rust_os::hlt_loop();
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}