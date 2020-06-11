#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};
#[cfg(test)] use rust_os::qemu::{exit_qemu, QemuExitCode};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! 
{
    use rust_os::memory;
    use x86_64::{structures::paging::MapperAllSizes, VirtAddr};

    println!("Starting up OS...");

    println!("Initializing modules...");

    rust_os::init();

    #[cfg(test)]
    test_main();
    #[cfg(test)]
    exit_qemu(QemuExitCode::Success);

    kernel_process();
}

fn kernel_process() -> !
{
    println!("OS ready. Hello. :)");
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