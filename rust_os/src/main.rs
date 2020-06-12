#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use rust_os::println;
use bootloader::{BootInfo, entry_point};
#[cfg(test)] use rust_os::qemu::{exit_qemu, QemuExitCode};
use rust_os::memory::{self, BootInfoFrameAllocator};
use x86_64::VirtAddr;
use rust_os::task::keyboard;
use rust_os::task::{ Task, executor::Executor };

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! 
{
    println!("Starting up OS...");

    println!("Initializing modules...");

    rust_os::init();

    use rust_os::allocator; 

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    println!("OS ready. Hello. :)");

    test_process();

    kernel_process();
}

fn kernel_process() -> !
{
    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[allow(unused)]
fn idle_loop() -> !
{
    rust_os::hlt_loop();
}

fn test_process()
{
    #[cfg(test)]
    test_main();
    #[cfg(test)]
    exit_qemu(QemuExitCode::Success);
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