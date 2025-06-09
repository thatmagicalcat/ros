use crate::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    vga_buffer::WRITER.lock().set_color(vga_buffer::Color::Red, vga_buffer::Color::Black);
    println!("{}", info);

    loop {  }
}
