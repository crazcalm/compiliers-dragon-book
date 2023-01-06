use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::process::ExitCode;
use std::str::Lines;

fn get_input_string(path: &str) -> String {
    let mut result = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut result).unwrap();
    result
}

struct InputFeed {
    cursor: i32,
    lineno: i32,
    index: usize,
    input: String,
}

impl InputFeed {
    fn new(input: String) -> Self {
        InputFeed {
            cursor: 0,
            lineno: 1,
            index: 0,
            input: input,
        }
    }

    fn get_char(&mut self) -> Option<char> {
        self.index += 1;

        let mut characters = self.input.chars();

        match characters.nth(self.index) {
            Some(input_char) => {
                if input_char.eq(&'\n') {
                    self.lineno += 1;
                    self.cursor = -1;
                    return Some(input_char);
                }

                self.cursor += 1;
                Some(input_char)
            }
            None => None,
        }
    }

    fn unget_char(&mut self) -> Option<char> {
        self.index -= 1;

        let mut characters = self.input.chars();
        match characters.nth(self.index) {
            Some(input_char) => {
                if input_char.eq(&'\n') {
                    self.lineno -= 1;
                }
                // Ignoring cursor backtrack edge case...
                self.cursor -= 1;

                Some(input_char)
            }
            None => None,
        }
    }
}

fn main() -> ExitCode {
    let mut args = env::args();
    let input_string: String;

    if args.len() > 1 {
        let path = args.nth(1).unwrap();
        input_string = get_input_string(&path);
    } else {
        eprintln!("No file to parse");
        return ExitCode::FAILURE;
    }

    let mut input_feed = InputFeed::new(input_string);
    for _ in 0..100 {
        print!("{:?}", input_feed.get_char());
    }
    println!();
    for _ in 0..5 {
        print!("{:?}", input_feed.unget_char());
    }

    return ExitCode::SUCCESS;
}
