use std::env;
use std::fmt::Display;
use std::fs;
use std::iter::Peekable;
use std::process::exit;

use anyhow::Result;

#[derive(Clone)]
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
    BANG,
    BangEqual,
    EQUAL,
    EqualEqual,
    GREATER,
    GreaterEqual,
    LESS,
    LessEqual,
    Slash,
    String(String),
    Number(String),
    IDENTIFIER(String),
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}
enum TokenTypeParseError {
    UnexpectedCharacter,
    UnterminatedString,
}

impl TokenType {
    fn to_lexeme(&self) -> String {
        let lexeme = match self {
            TokenType::LeftParen => format!("( null"),
            TokenType::RightParen => format!(") null"),
            TokenType::LeftBrace => format!("{{ null"),
            TokenType::RightBrace => format!("}} null"),
            TokenType::EOF => format!(" null"),
            TokenType::COMMA => format!(", null"),
            TokenType::DOT => format!(". null"),
            TokenType::MINUS => format!("- null"),
            TokenType::PLUS => format!("+ null"),
            TokenType::SEMICOLON => format!("; null"),
            TokenType::STAR => format!("* null"),
            TokenType::BANG => format!("! null"),
            TokenType::BangEqual => format!("!= null"),
            TokenType::EQUAL => format!("= null"),
            TokenType::EqualEqual => format!("== null"),
            TokenType::GREATER => format!("> null"),
            TokenType::GreaterEqual => format!(">= null"),
            TokenType::LESS => format!("< null"),
            TokenType::LessEqual => format!("<= null"),
            TokenType::Slash => format!("/ null"),
            TokenType::String(s) => format!("\"{s}\" {s}"),
            TokenType::Number(n) => {
                let s = format_number_string(n);

                format!("{n} {s}")
            }
            TokenType::IDENTIFIER(s) => format!("{s} null"),
            TokenType::AND => format!("and null"),
            TokenType::CLASS => format!("class null"),
            TokenType::ELSE => format!("else null"),
            TokenType::FALSE => format!("false null"),
            TokenType::FOR => format!("for null"),
            TokenType::FUN => format!("fun null"),
            TokenType::IF => format!("if null"),
            TokenType::NIL => format!("nil null"),
            TokenType::OR => format!("or null"),
            TokenType::PRINT => format!("print null"),
            TokenType::RETURN => format!("return null"),
            TokenType::SUPER => format!("super null"),
            TokenType::THIS => format!("this null"),
            TokenType::TRUE => format!("true null"),
            TokenType::VAR => format!("var null"),
            TokenType::WHILE => format!("while null"),
        };
        lexeme.to_owned()
    }

    fn from_chars(
        current: &char,
        chars: &mut Peekable<std::str::Chars>,
    ) -> Result<TokenType, TokenTypeParseError> {
        match current {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::COMMA),
            '.' => Ok(TokenType::DOT),
            '-' => Ok(TokenType::MINUS),
            '+' => Ok(TokenType::PLUS),
            ';' => Ok(TokenType::SEMICOLON),
            '*' => Ok(TokenType::STAR),
            '!' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::BangEqual)
            }
            '!' => Ok(TokenType::BANG),
            '=' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::EqualEqual)
            }
            '=' => Ok(TokenType::EQUAL),
            '>' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::GreaterEqual)
            }
            '>' => Ok(TokenType::GREATER),
            '<' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::LessEqual)
            }
            '<' => Ok(TokenType::LESS),
            '/' => Ok(TokenType::Slash),
            '\"' => {
                let mut content = String::new();

                while let Some(c) = chars.next() {
                    if c == '\"' {
                        return Ok(TokenType::String(content));
                    }
                    content.push(c);
                }

                Err(TokenTypeParseError::UnterminatedString)
            }

            c if c.is_digit(10) => {
                let mut number_string = String::new();
                number_string.push(*c);

                while let Some(n) = chars.next_if(|x| x.is_digit(10)) {
                    number_string.push(n);
                }

                'rest: while let Some(&n) = chars.peek() {
                    if n == '.' && chars.clone().nth(1).is_some_and(|n| n.is_digit(10)) {
                        number_string.push(n);
                        chars.next();
                        while let Some(rest) = chars.peek() {
                            if rest.is_digit(10) {
                                number_string.push(*rest);
                                chars.next();
                            } else {
                                break 'rest;
                            }
                        }
                    } else {
                        break 'rest;
                    }
                }

                Ok(TokenType::Number(number_string))
            }
            c if c.is_alphabetic() || c == &'_' => {
                let mut identifier = String::from(*c);
                while let Some(char) = chars.next_if(|c| c.is_alphanumeric() || c == &'_') {
                    identifier.push(char);
                }
                let token_type = match identifier.as_str() {
                    "and" => TokenType::AND,
                    "class" => TokenType::CLASS,
                    "else" => TokenType::ELSE,
                    "false" => TokenType::FALSE,
                    "for" => TokenType::FOR,
                    "fun" => TokenType::FUN,
                    "if" => TokenType::IF,
                    "nil" => TokenType::NIL,
                    "or" => TokenType::OR,
                    "print" => TokenType::PRINT,
                    "return" => TokenType::RETURN,
                    "super" => TokenType::SUPER,
                    "this" => TokenType::THIS,
                    "true" => TokenType::TRUE,
                    "var" => TokenType::VAR,
                    "while" => TokenType::WHILE,
                    _ => TokenType::IDENTIFIER(identifier),
                };
                Ok(token_type)
            }
            _ => Err(TokenTypeParseError::UnexpectedCharacter),
        }
    }
}
fn format_number_string(string: &String) -> String {
    let mut s = String::from(string);
    if !s.contains('.') {
        s.push_str(".0")
    } else if s.contains(".00") {
        let pos = s.find('.').unwrap();

        let striped = s.split_at(pos);
        let mut s = String::from(striped.0);
        s.push_str(".0");
        return s;
    }
    s
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
            TokenType::BANG => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::EQUAL => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::GREATER => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::LESS => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Slash => "SLASH",
            TokenType::String(_) => "STRING",
            TokenType::Number(_) => "NUMBER",
            TokenType::IDENTIFIER(_) => "IDENTIFIER",
            TokenType::AND => "AND",
            TokenType::CLASS => "CLASS",
            TokenType::ELSE => "ELSE",
            TokenType::FALSE => "FALSE",
            TokenType::FOR => "FOR",
            TokenType::FUN => "FUN",
            TokenType::IF => "IF",
            TokenType::NIL => "NIL",
            TokenType::OR => "OR",
            TokenType::PRINT => "PRINT",
            TokenType::RETURN => "RETURN",
            TokenType::SUPER => "SUPER",
            TokenType::THIS => "THIS",
            TokenType::TRUE => "TRUE",
            TokenType::VAR => "VAR",
            TokenType::WHILE => "WHILE",
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
        write!(f, "{} {}", self.token_type, self.lexeme)
    }
}

impl Token {
    fn new(token_type: TokenType) -> Token {
        Token {
            token_type: token_type.clone(),
            lexeme: token_type.to_lexeme(),
        }
    }
}

fn skip_char(char: char) -> bool {
    match char {
        ' ' => true,
        '\n' => true,
        '\r' => true,
        '\t' => true,
        _ => false,
    }
}

fn tokenize(input: String, line: usize) -> Vec<Result<String, String>> {
    let mut token_vec: Vec<Result<String, String>> = vec![];
    let mut iter = input.chars().peekable();
    while let Some(token) = iter.next() {
        if skip_char(token) {
            continue;
        }
        if token == '/' && iter.peek() == Some(&'/') {
            break;
        }
        let token_type = TokenType::from_chars(&token, &mut iter);
        match token_type {
            Ok(token_type) => token_vec.push(Ok(format!("{}", Token::new(token_type)))),
            Err(TokenTypeParseError::UnexpectedCharacter) => token_vec.push(Err(format!(
                "[line {}] Error: Unexpected character: {}",
                line, token
            ))),
            Err(TokenTypeParseError::UnterminatedString) => {
                token_vec.push(Err(format!("[line {line}] Error: Unterminated string.")))
            }
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
