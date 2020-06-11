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
    println!("Starting up OS...");

    println!("Initializing modules...");

    rust_os::init();

    memory_mapping(boot_info);

    test_process();

    kernel_process();
}

fn kernel_process() -> !
{
    println!("OS ready. Hello. :)");
    rust_os::hlt_loop();
}

fn test_process()
{
    #[cfg(test)]
    test_main();
    #[cfg(test)]
    exit_qemu(QemuExitCode::Success);
}

fn memory_mapping(boot_info: &'static BootInfo)
{
    use rust_os::memory;
    use x86_64::{structures::paging::MapperAllSizes, VirtAddr};

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    let mapper = unsafe { memory::init(phys_mem_offset) };

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        // new: use the `mapper.translate_addr` method
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }
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