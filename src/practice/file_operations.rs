use std::fs;
use std::io::{self, Write};

pub fn file_operations() -> io::Result<()> {
    let path = "sample.txt";
    let mut file = fs::File::create(path)?;
    writeln!(file, "Hello, world!")?;
    println!("ファイルが作成されました: {}", path);
    Ok(())
}
