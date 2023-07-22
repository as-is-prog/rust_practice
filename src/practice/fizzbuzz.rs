use crate::console::io;

fn fizz_buzz(start: i32, count: i32) -> Result<Vec<String>, String> {
    if count < 1 {
        return Err("must be count > 0".to_string());
    }

    let result = (start..(start + count))
        .map(|n| match (n % 3 == 0, n % 5 == 0) {
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            _ => n.to_string(),
        })
        .collect();

    Ok(result)
}

pub fn fizzbuzz_main() {
    io::println_str("[FizzBuzz]").unwrap();

    io::print_str("FizzBuzz Start Number (def: 1): ").unwrap();
    let start = io::read_i32().unwrap_or(1);

    io::print_str("FizzBuzz Count:  (def: 15)").unwrap();
    let count = io::read_i32().unwrap_or(15);

    let fizz_buzz_result = fizz_buzz(start, count).unwrap();
    fizz_buzz_result.iter().for_each(|s| println!("{}", s));
}
