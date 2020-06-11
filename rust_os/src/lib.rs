#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)] // at the top of the file

extern crate alloc;

pub mod vga_driver;
#[macro_use] pub mod serial_driver;
pub mod test_utilities;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod qemu;
pub mod allocator;

use core::panic::PanicInfo;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


pub fn test_runner(tests: &[&dyn test_utilities::Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel_main);

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    qemu::exit_qemu(qemu::QemuExitCode::Success);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
