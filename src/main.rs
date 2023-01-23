use std::env;
use std::fs;
use std::io::{self, BufRead};

use arboard::Clipboard;

fn main() {
    let mut args = env::args().skip(1);
    match args.len() {
        0 => handle_clipboard(),
        1 => match args.next() {
            Some(arg) => {
                match arg.as_str() {
                    "-i" => handle_stdin(),
                    _ => handle_file(arg),
                };
            }
            None => handle_clipboard(),
        }
        _ => for arg in args {
            println!("\x1b[92m=> {}\x1b[0m", arg);
            handle_file(arg);
        }
    };
}

fn handle_clipboard() {
    let input = read_clipboard();
    let output = process_input(input);
    println!("{}", output);
}

fn handle_file(file: String) {
    match fs::read_to_string(file) {
        Ok(input) => {
            let output = process_input(input);
            println!("{}", output);
        }
        Err(e) => println!("Error: {}", e),
    }
}

fn handle_stdin() {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(x) => {
                let output = process_input(x);
                println!("{}", output);
            }
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };
    }
}

fn process_input(input: String) -> String {
    input
        .replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
}

fn read_clipboard() -> String {
    match Clipboard::new().unwrap().get_text() {
        Ok(val) => val,
        Err(_) => {
            println!("Failed to get clipboard text");
            return "".to_string();
        }
    }
}
