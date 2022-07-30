// kamil_os with Rust
// https://os.phil-opp.com/freestanding-rust-binary/

// An example for such a bare metal environment is the thumbv7em-none-eabihf target triple,
// which describes an embedded ARM system. The details are not important,
// all that matters is that the target triple has no underlying operating system,
// which is indicated by the none in the target triple.
// rustup target add thumbv7em-none-eabihf

// Command to build for bare-metal
// cargo build --target thumbv7em-none-eabihf

// main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//static HELLO: &[u8] = b"Hello World!";

/// Overwriting the operating system entry point with our own _start function:
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    vga_buffer::print_something();
    loop {}
}
