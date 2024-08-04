use std::env;
use std::fmt::Display;
use std::fs;
use std::process::exit;
use std::str::FromStr;

use anyhow::Result;

#[derive(Clone, Copy)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    EOF,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
}
struct TokenTypeParseError;
impl FromStr for TokenType {
    type Err = TokenTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(TokenType::EOF),
            "(" => Ok(TokenType::LeftParen),
            ")" => Ok(TokenType::RightParen),
            "{" => Ok(TokenType::LeftBrace),
            "}" => Ok(TokenType::RightBrace),
            "," => Ok(TokenType::COMMA),
            "." => Ok(TokenType::DOT),
            "-" => Ok(TokenType::MINUS),
            "+" => Ok(TokenType::PLUS),
            ";" => Ok(TokenType::SEMICOLON),
            "*" => Ok(TokenType::STAR),
            _ => Err(TokenTypeParseError),
        }
    }
}

impl TokenType {
    fn to_lexeme(&self) -> String {
        let lexeme = match self {
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::EOF => "",
            TokenType::COMMA => ",",
            TokenType::DOT => ".",
            TokenType::MINUS => "-",
            TokenType::PLUS => "+",
            TokenType::SEMICOLON => ";",
            TokenType::STAR => "*",
        };
        lexeme.to_owned()
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self {
            TokenType::EOF => "EOF",
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::COMMA => "COMMA",
            TokenType::DOT => "DOT",
            TokenType::MINUS => "MINUS",
            TokenType::PLUS => "PLUS",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::STAR => "STAR",
        };
        write!(f, "{}", name)
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} null", self.token_type, self.lexeme)
    }
}

impl Token {
    fn new(token_type: TokenType) -> Token {
        Token {
            token_type,
            lexeme: token_type.to_lexeme(),
        }
    }
}

fn tokenize(input: String, line: usize) -> Vec<Result<String, String>> {
    let mut token_vec: Vec<Result<String, String>> = vec![];
    for token in input.chars() {
        if token == ' ' || token == '\n' {
            continue;
        }
        let token_type = TokenType::from_str(&token.to_string());
        match token_type {
            Ok(token_type) => token_vec.push(Ok(format!("{}", Token::new(token_type)))),
            Err(_) => token_vec.push(Err(format!(
                "[line {}] Error: Unexpected character: {}",
                line, token
            ))),
        };
    }

    token_vec
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);

                String::new()
            });

            let token_lines: Vec<Result<String, String>> = file_contents
                .lines()
                .enumerate()
                .map(|(number, line)| tokenize(line.to_owned(), number + 1))
                .flatten()
                .collect();
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
            let eof_token = Token::new(TokenType::EOF);

            println!("{eof_token}");
            exit(exit_code);
        }
        _ => {
            eprintln!("Unknown command: {}", command);

            return;
        }
    }
}
