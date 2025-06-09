pub trait TestFn {
    fn run(&self);
}

impl<T: Fn()> TestFn for T {
    fn run(&self) {
        crate::serial_print!("{}...", core::any::type_name::<T>());
        self();
        crate::serial_println!("[ok]");
    }
}
