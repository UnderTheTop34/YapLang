mod executer;
mod util;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::executer::executer::Executer;
use crate::util::remove_comments::remove_comments;

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
            let mut executer = Executer::new(file_contents);
            executer.execute(|x| print!("{}", x));

        }

        "rmc" /*rm comments*/ => {
            if args.len() <= 2 {
                println!("Missing argument - file required");
                return;
            }

            let file_contents = open_file(&args[2]);
            let n_file_contents = remove_comments(file_contents.join("\n"));
            println!("{}", n_file_contents);
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