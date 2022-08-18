// kamil_os with Rust
// https://os.phil-opp.com/

// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(kamil_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::boxed::Box;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kamil_os::{
    allocator,
    memory::{self, BootInfoFrameAllocator},
    print, println,
};
use x86_64::VirtAddr;

//------------------------------------------

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    print!("Hello World{}\n", "!");
    println!("0b{:08b}/0b{:08b} = 0d{}", 1, 3, 1.0 / 3.0);

    // Interrupts initialization
    kamil_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    let _x = Box::new(41);

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    kamil_os::hlt_loop();
}

//------------------------------------------

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kamil_os::hlt_loop();
}

/// This function is called on panic during test
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kamil_os::test_panic_handler(info)
}
