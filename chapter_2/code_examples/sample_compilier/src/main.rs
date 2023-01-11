use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

mod feed;

use crate::feed::Feed;

fn get_input_string(path: &str) -> String {
    let mut result = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut result).unwrap();
    result
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

    let mut input_feed = feed::InputFeed::new(input_string);
    for _ in 0..10 {
        print!("{:?}", input_feed.get_char());
    }
    println!();
    for _ in 0..5 {
        print!("{:?}", input_feed.unget_char());
    }

    return ExitCode::SUCCESS;
}
