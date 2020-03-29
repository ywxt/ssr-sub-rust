use serde::export::fmt::Debug;
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        println!("\u{001b}[31merror:{}\u{001b}[0m",format_args!($($arg)*))
    };
}
#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {
        println!("\u{001b}[33mwarning:{}\u{001b}[0m",format_args!($($arg)*))
    };
}

pub fn print_group(name: &str, items: &impl IntoIterator<Item = dyn Debug>) {}
