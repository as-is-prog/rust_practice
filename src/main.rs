mod console;
mod practice;
use console::io;
use practice::{fizzbuzz, shiftup};

fn main() {
    io::println_str("[Practice]").unwrap();
    io::println_str("0: shiftup").unwrap();
    io::println_str("1: fizzbuzz").unwrap();

    match io::read_i32().unwrap_or(-1) {
        0 => shiftup::shiftup_main(),
        1 => fizzbuzz::fizzbuzz_main(),
        _ => println!("Invalid input."),
    }
}
