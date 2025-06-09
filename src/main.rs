#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test"]

mod test;
mod panic;
mod serial;
mod vga_buffer;

#[macro_use]
mod macros;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test();

    println!("hello, world!!");

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn test::TestFn]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
}

#[test_case]
#[allow(clippy::eq_op)]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
