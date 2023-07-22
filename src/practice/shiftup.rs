use crate::console::io;

pub fn shiftup_main() {
    let mut holydays = Vec::<i32>::new();

    loop {
        io::println_str("Input requset holydays(exit non input.): ").unwrap();

        match io::read_i32() {
            Ok(n) => holydays.push(n),
            Err(_) => break,
        }
    }

    io::println_str("[Holydays]").unwrap();
    holydays.iter().for_each(|n| println!("{}", n));
}
