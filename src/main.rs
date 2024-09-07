pub mod parser;
pub mod scanner;

use parser::primary;
use scanner::{tokenize, Token};
use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);

        String::new()
    });
    let token_lines = file_contents
        .lines()
        .enumerate()
        .flat_map(|(number, line)| tokenize(line, number + 1))
        .peekable();

    match command.as_str() {
        "tokenize" => {
            let mut exit_code = 0;
            for token_line in token_lines {
                match token_line {
                    Ok(line) => println!("{line}"),
                    Err(err) => {
                        exit_code = 65;
                        eprintln!("{err}")
                    }
                }
            }
            let eof_token = Token::Eof;

            println!("{eof_token}");
            exit(exit_code);
        }
        "parse" => {
            let exprs = primary(token_lines);

            for expr in exprs {
                match expr {
                    Ok(expr) => println!("{expr}"),
                    Err(_) => eprintln!("error"),
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
