#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(boronos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use boronos::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    boronos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    boronos::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    boronos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    boronos::hlt_loop()
}
