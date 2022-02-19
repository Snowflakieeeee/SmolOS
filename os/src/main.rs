#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
extern crate alloc;

use os::{
    println,
    task::{executor::Executor, keyboard, Task},
    vga_buffer::Color,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!(FG: Color::Yellow, "Hello World!");
    println!(FG: Color::Yellow, "INFO: Kernel Started");
    println!(
        FG: Color::DarkGray,
        "----------------------------------------\n"
    );
    os::init(boot_info);

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
