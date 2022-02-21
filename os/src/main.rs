#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
extern crate alloc;

use os::task::{executor::Executor, Task};

mod kernel;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    os::init(boot_info);
    os::init_screens();

    let mut executor = Executor::new();
    executor.spawn(Task::new(kernel::handle_main()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::println!("{}", info);
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
