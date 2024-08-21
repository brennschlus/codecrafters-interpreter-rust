use std::fmt::Display;
use std::iter::Peekable;

use anyhow::Result;

#[derive(Clone)]
pub enum TokenType {
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
                let mut number_string = String::from(*c);

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
            TokenType::EOF => "EOF  null",
            TokenType::LeftParen => "LEFT_PAREN ( null",
            TokenType::RightParen => "RIGHT_PAREN ) null",
            TokenType::LeftBrace => "LEFT_BRACE { null",
            TokenType::RightBrace => "RIGHT_BRACE } null",
            TokenType::COMMA => "COMMA , null",
            TokenType::DOT => "DOT . null",
            TokenType::MINUS => "MINUS - null",
            TokenType::PLUS => "PLUS + null",
            TokenType::SEMICOLON => "SEMICOLON ; null",
            TokenType::STAR => "STAR * null",
            TokenType::BANG => "BANG ! null",
            TokenType::BangEqual => "BANG_EQUAL = null",
            TokenType::EQUAL => "EQUAL = null",
            TokenType::EqualEqual => "EQUAL_EQUAL == null",
            TokenType::GREATER => "GREATER > null",
            TokenType::GreaterEqual => "GREATER_EQUAL >= null",
            TokenType::LESS => "LESS < null",
            TokenType::LessEqual => "LESS_EQUAL <= null",
            TokenType::Slash => "SLASH / null",
            TokenType::String(s) => &(format!("STRING \"{s}\" {s}")),
            TokenType::Number(n) => &format!("NUMBER {n} {}", format_number_string(n)),
            TokenType::IDENTIFIER(i) => &format!("IDENTIFIER {i} null"),
            TokenType::AND => "AND and null",
            TokenType::CLASS => "CLASS class null",
            TokenType::ELSE => "ELSE else null",
            TokenType::FALSE => "FALSE false null",
            TokenType::FOR => "FOR for null",
            TokenType::FUN => "FUN fun null",
            TokenType::IF => "IF if null",
            TokenType::NIL => "NIL nil null",
            TokenType::OR => "OR or null",
            TokenType::PRINT => "PRINT print null",
            TokenType::RETURN => "RETURN return null",
            TokenType::SUPER => "SUPER super null",
            TokenType::THIS => "THIS this null",
            TokenType::TRUE => "TRUE true null",
            TokenType::VAR => "VAR var null",
            TokenType::WHILE => "WHILE while null",
        };
        write!(f, "{}", name)
    }
}

pub struct Token {
    token_type: TokenType,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type)
    }
}

impl Token {
    pub fn new(token_type: TokenType) -> Token {
        Token { token_type }
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

pub fn tokenize(input: String, line: usize) -> Vec<Result<String, String>> {
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
