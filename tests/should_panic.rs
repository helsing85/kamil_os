// tests/should_panic.rs

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kamil_os::serial_print;
use kamil_os::{exit_qemu, serial_println, QemuExitCode};

//------------------------------------------

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

//------------------------------------------

// Only the first function is executed
// because the execution cannot continue
// after the panic handler has been called.
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

//------------------------------------------

#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
