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
fn kernel_main(boot_info: &'static BootInfo) -> ! 
{
    use rust_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Starting up OS...");

    println!("Initializing modules...");

    rust_os::init();

    println!("Translating memory mapping...");

    println!("Physical memory offset is {}", boot_info.physical_memory_offset);

    println!("Loading level 4 page table...");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();
    #[cfg(test)]
    rust_os::exit_qemu(rust_os::QemuExitCode::Success);

    println!("Tasks finished. Going idle.");

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