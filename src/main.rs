#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

mod panic;
mod vga_buffer;

#[macro_use]
mod macros;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("hello, world!!");

    loop {}
}
