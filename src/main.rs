use arboard::Clipboard;
use std::env;
use std::io::{self, BufRead};

fn main() {
    let mut args = env::args();
    if args.len() > 1 && args.nth(1).unwrap() == "-i" {
        handle_stdin();
    } else {
        handle_clipboard();
    }
}

fn handle_clipboard() {
    let input = read_clipboard();
    let output = process_input(input);
    println!("{}", output);
}

fn handle_stdin() {
    for line in io::stdin().lock().lines() {
        let input = line.unwrap();
        let output = process_input(input);
        println!("{}", output);
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
