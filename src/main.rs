mod console;
mod practice;

use practice::{file_operations, fizzbuzz, shiftup};

fn main() -> std::io::Result<()> {
    console::io::println_str("[Practice]").unwrap();
    console::io::println_str("0: shiftup").unwrap();
    console::io::println_str("1: fizzbuzz").unwrap();
    console::io::println_str("2: file_operations").unwrap();

    match console::io::read_i32().unwrap_or(-1) {
        0 => shiftup::shiftup_main(),
        1 => fizzbuzz::fizzbuzz_main(),
        2 => file_operations::file_operations()?,
        _ => println!("Invalid input."),
    }
    Ok(())
}
