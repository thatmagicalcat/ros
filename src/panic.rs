use crate::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga_buffer::WRITER
        .lock()
        .set_color(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("{}", info);

    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    loop {}
}
