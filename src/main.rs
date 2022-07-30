// kamil_os with Rust
// https://os.phil-opp.com/

// main.rs
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Overwriting the operating system entry point with our own _start function:
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    print!("Hello World{}\n", "!");
    println!("0b{:08b}/0b{:08b} = 0d{}", 1, 3, 1.0 / 3.0);

    //panic!("Some panic message");

    #[cfg(test)]
    test_main();

    loop {}
}

//------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    //exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
