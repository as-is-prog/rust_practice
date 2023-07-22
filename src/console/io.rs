#![allow(dead_code)]

use std::io;

#[derive(Debug)]
pub enum ConsoleIoError {
    IoError(io::Error),
    ParseError(std::num::ParseIntError),
}

pub fn read_i32() -> Result<i32, ConsoleIoError> {
    let mut buf = String::new();

    match io::stdin().read_line(&mut buf) {
        Ok(_) => match buf.trim().parse::<i32>() {
            Ok(n) => Ok(n),
            Err(e) => Err(ConsoleIoError::ParseError(e)),
        },
        Err(e) => Err(ConsoleIoError::IoError(e)),
    }
}

pub fn print_str(s: &str) -> Result<(), io::Error> {
    print!("{}", s);
    match std::io::Write::flush(&mut std::io::stdout()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn println_str(s: &str) -> Result<(), io::Error> {
    println!("{}", s);
    match std::io::Write::flush(&mut std::io::stdout()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn flush() -> Result<(), io::Error> {
    match std::io::Write::flush(&mut std::io::stdout()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
