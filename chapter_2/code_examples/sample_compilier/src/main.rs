mod feed;
mod tokens;
mod symbols;
mod lexan;
mod parser;

use std::env;
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use crate::symbols::SymbolTable;
use crate::lexan::lexan;
use crate::parser::Parser;


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

    let input_feed = feed::InputFeed::new(input_string);
    let symbol_table = SymbolTable::new();

    let processed_tokens = lexan(input_feed, symbol_table);
    println!("Processed Tokens: {:?}", &processed_tokens);

    let mut parser = Parser::new(processed_tokens);
    parser.parse();

    return ExitCode::SUCCESS;
}
