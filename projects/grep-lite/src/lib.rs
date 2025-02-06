use ansi_term::Colour::Red;
use std::io::{stderr, Write};
use std::process::exit;
pub mod stdio;
pub mod print;
pub fn stderr_out(msg: &str) {
    let stderr = stderr();
    let mut handle = stderr.lock();
    writeln!(handle, "{}", Red.paint(msg)).unwrap();
}

pub fn show_usage() {
    println!("[Usage]:");
    println!("grep-lite [PATTERN]");
    println!();
    println!();
    println!("[Options]:");
    println!("[1] -f [FilePath]");
    println!();
    println!();
    println!("[Examples]:");
    println!("[1] grep-lite abcd -f file1.txt");
    println!("[2] echo \"hello\" | grep-lite abcd ");
    println!("[3] cat file1.txt | grep-lite abcd ");
    println!();
    exit(0);
}
