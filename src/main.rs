use std::env;
use std::fmt::Display;
use std::fs;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Clone, Copy)]
enum TokenType {
    LeftParen,
    RightParen,
    EOF,
}
struct TokenTypeParseError;
impl FromStr for TokenType {
    type Err = TokenTypeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(TokenType::EOF),
            "(" => Ok(TokenType::LeftParen),
            ")" => Ok(TokenType::RightParen),
            _ => Err(TokenTypeParseError),
        }
    }
}

impl TokenType {
    fn to_lexeme(&self) -> String {
        match self {
            &TokenType::LeftParen => "(".to_owned(),
            &TokenType::RightParen => ")".to_owned(),
            &TokenType::EOF => "".to_owned(),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match &self {
            TokenType::EOF => "EOF",
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
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

fn tokenize(input: String) {
    for token in input.chars() {
        if token == ' ' || token == '\n' {
            return;
        }
        let token_type = TokenType::from_str(&token.to_string());
        match token_type {
            Ok(token_type) => println!("{}", Token::new(token_type)),
            Err(_) => eprintln!("wrong token"),
        }
    }

    let eof_token = Token::new(TokenType::EOF);
    println!("{}", eof_token);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            tokenize(file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
