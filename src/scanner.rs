use std::fmt::Display;
use std::iter::Peekable;

use anyhow::Result;

#[derive(Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Eof,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Slash,
    String(String),
    Number(String),
    Identifier(String),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}
pub(crate) enum TokenParseError {
    UnexpectedCharacter,
    UnterminatedString,
}

impl Token {
    fn from_chars(
        current: &char,
        chars: &mut Peekable<std::str::Chars>,
    ) -> Result<Token, TokenParseError> {
        match current {
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            ',' => Ok(Token::Comma),
            '.' => Ok(Token::Dot),
            '-' => Ok(Token::Minus),
            '+' => Ok(Token::Plus),
            ';' => Ok(Token::Semicolon),
            '*' => Ok(Token::Star),
            '!' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(Token::BangEqual)
            }
            '!' => Ok(Token::Bang),
            '=' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(Token::EqualEqual)
            }
            '=' => Ok(Token::Equal),
            '>' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(Token::GreaterEqual)
            }
            '>' => Ok(Token::Greater),
            '<' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(Token::LessEqual)
            }
            '<' => Ok(Token::Less),
            '/' => Ok(Token::Slash),
            '\"' => {
                let mut content = String::new();

                for c in chars {
                    if c == '\"' {
                        return Ok(Token::String(content));
                    }
                    content.push(c);
                }

                Err(TokenParseError::UnterminatedString)
            }

            c if c.is_ascii_digit() => {
                let mut number_string = String::from(*c);

                while let Some(n) = chars.next_if(|x| x.is_ascii_digit()) {
                    number_string.push(n);
                }

                'rest: while let Some(&n) = chars.peek() {
                    if n == '.' && chars.clone().nth(1).is_some_and(|n| n.is_ascii_digit()) {
                        number_string.push(n);
                        chars.next();
                        while let Some(rest) = chars.peek() {
                            if rest.is_ascii_digit() {
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

                Ok(Token::Number(number_string))
            }
            c if c.is_alphabetic() || c == &'_' => {
                let mut identifier = String::from(*c);
                while let Some(char) = chars.next_if(|c| c.is_alphanumeric() || c == &'_') {
                    identifier.push(char);
                }
                let token_type = match identifier.as_str() {
                    "and" => Token::And,
                    "class" => Token::Class,
                    "else" => Token::Else,
                    "false" => Token::False,
                    "for" => Token::For,
                    "fun" => Token::Fun,
                    "if" => Token::If,
                    "nil" => Token::Nil,
                    "or" => Token::Or,
                    "print" => Token::Print,
                    "return" => Token::Return,
                    "super" => Token::Super,
                    "this" => Token::This,
                    "true" => Token::True,
                    "var" => Token::Var,
                    "while" => Token::While,
                    _ => Token::Identifier(identifier),
                };
                Ok(token_type)
            }
            _ => Err(TokenParseError::UnexpectedCharacter),
        }
    }
}
pub fn format_number_string(string: &String) -> String {
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_string: String;
        let name = match &self {
            Token::Eof => "EOF  null",
            Token::LeftParen => "LEFT_PAREN ( null",
            Token::RightParen => "RIGHT_PAREN ) null",
            Token::LeftBrace => "LEFT_BRACE { null",
            Token::RightBrace => "RIGHT_BRACE } null",
            Token::Comma => "COMMA , null",
            Token::Dot => "DOT . null",
            Token::Minus => "MINUS - null",
            Token::Plus => "PLUS + null",
            Token::Semicolon => "SEMICOLON ; null",
            Token::Star => "STAR * null",
            Token::Bang => "BANG ! null",
            Token::BangEqual => "BANG_EQUAL != null",
            Token::Equal => "EQUAL = null",
            Token::EqualEqual => "EQUAL_EQUAL == null",
            Token::Greater => "GREATER > null",
            Token::GreaterEqual => "GREATER_EQUAL >= null",
            Token::Less => "LESS < null",
            Token::LessEqual => "LESS_EQUAL <= null",
            Token::Slash => "SLASH / null",
            Token::String(s) => {
                temp_string = format!("STRING \"{s}\" {s}");
                &temp_string
            }
            Token::Number(n) => {
                temp_string = format!("NUMBER {n} {}", format_number_string(n));
                &temp_string
            }
            Token::Identifier(i) => {
                temp_string = format!("IDENTIFIER {i} null");
                &temp_string
            }
            Token::And => "AND and null",
            Token::Class => "CLASS class null",
            Token::Else => "ELSE else null",
            Token::False => "FALSE false null",
            Token::For => "FOR for null",
            Token::Fun => "FUN fun null",
            Token::If => "IF if null",
            Token::Nil => "NIL nil null",
            Token::Or => "OR or null",
            Token::Print => "PRINT print null",
            Token::Return => "RETURN return null",
            Token::Super => "SUPER super null",
            Token::This => "THIS this null",
            Token::True => "TRUE true null",
            Token::Var => "VAR var null",
            Token::While => "WHILE while null",
        };
        write!(f, "{}", name)
    }
}

fn skip_char(char: char) -> bool {
    matches!(char, ' ' | '\n' | '\r' | '\t')
}

pub fn tokenize(input: &str, line: usize) -> Vec<Result<Token, String>> {
    let mut token_vec: Vec<Result<Token, String>> = vec![];
    let mut iter = input.chars().peekable();
    while let Some(char) = iter.next() {
        if skip_char(char) {
            continue;
        }
        if char == '/' && iter.peek() == Some(&'/') {
            break;
        }
        let token = Token::from_chars(&char, &mut iter);
        match token {
            Ok(token) => token_vec.push(Ok(token)),
            Err(TokenParseError::UnexpectedCharacter) => token_vec.push(Err(format!(
                "[line {}] Error: Unexpected character: {}",
                line, char
            ))),
            Err(TokenParseError::UnterminatedString) => {
                token_vec.push(Err(format!("[line {line}] Error: Unterminated string.")))
            }
        };
    }

    token_vec
}
