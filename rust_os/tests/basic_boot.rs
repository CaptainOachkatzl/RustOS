#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn rust_os::test_utilities::Testable]) 
{
    rust_os::test_runner(tests);
    rust_os::exit_qemu(rust_os::QemuExitCode::Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    rust_os::test_panic_handler(info);
}

#[test_case]
fn test_println() 
{
    println!("test_println output");
}
