# LPLang — CLI Tool

use std::io::{stdin, stdout, BufRead, Write}
use std::env::args

fn main() {
    let args: Vec<String> = args().collect()
    let name = args.get(1).map(|s| s.as_str()).unwrap_or("world")
    print!("Hello, {name}!")
}