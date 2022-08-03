// kamil_os with Rust
// https://os.phil-opp.com/

// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(kamil_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use kamil_os::{print, println};

//------------------------------------------

/// Overwriting the operating system entry point with our own _start function:
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    print!("Hello World{}\n", "!");
    println!("0b{:08b}/0b{:08b} = 0d{}", 1, 3, 1.0 / 3.0);

    kamil_os::init();

    //panic!("Some panic message");

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

//------------------------------------------

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is called on panic during test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kamil_os::test_panic_handler(info)
}
