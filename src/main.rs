use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use colorize;
use colorize::AnsiColor;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Missing argument, no file specified");
        return;
    }

    let f = File::open(args.last().unwrap()).expect("Could not open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Should be able to read line");
        println!("Line: {}", line);
    }
}
