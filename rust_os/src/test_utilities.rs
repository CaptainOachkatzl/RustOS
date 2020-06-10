pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where 
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
