mod executer;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use colorize;
use crate::executer::executer::Executer;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Missing argument - printing help screen");

        return;
    }

    let main_command = args[1].clone();

    match main_command.as_ref() {
        "run" => {
            if args.len() <= 2 {
                println!("Missing argument - file required");
                return;
            }

            let file_contents = open_file(&args[2]);
            println!("{:?}", file_contents);
            let mut executer = Executer::new(file_contents);
            executer.execute({|x| print!("{}", x)});

        }

        _ => print_help()
    }

}

fn print_help() {

}

fn open_file(path: &str) -> Vec<String> {
    let f = File::open(path).expect("Could not open file");
    let f = BufReader::new(f);

    let text = f.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    text
}