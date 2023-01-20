use arboard::Clipboard;
use std::env;
use std::io::prelude::*;
use std::str;

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
    println!("\n{}", output);
}

fn handle_stdin() {
    let mut buffer = Vec::with_capacity(256);
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    match handle.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(e) => {
            println!("error: {}", e);
        }
    }

    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    match handle.write_all(&{ process_vec(&buffer) }) {
        Ok(_) => (),
        Err(e) => {
            println!("error: {}", e);
        }
    }
}

fn process_input(input: String) -> String {
    input
        .replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
}

fn process_vec(input: &Vec<u8>) -> Vec<u8> {
    let s = match str::from_utf8(input) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    process_input(s.to_string()).into_bytes()
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
