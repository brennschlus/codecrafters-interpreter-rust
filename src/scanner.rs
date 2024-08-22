use std::fmt::Display;
use std::iter::Peekable;

use anyhow::Result;

#[derive(Clone)]
pub enum TokenType {
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
pub(crate) enum TokenTypeParseError {
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
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::Semicolon),
            '*' => Ok(TokenType::Star),
            '!' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::BangEqual)
            }
            '!' => Ok(TokenType::Bang),
            '=' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::EqualEqual)
            }
            '=' => Ok(TokenType::Equal),
            '>' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::GreaterEqual)
            }
            '>' => Ok(TokenType::Greater),
            '<' if { chars.peek().is_some_and(|c| c == &'=') } => {
                chars.next();
                Ok(TokenType::LessEqual)
            }
            '<' => Ok(TokenType::Less),
            '/' => Ok(TokenType::Slash),
            '\"' => {
                let mut content = String::new();

                for c in chars {
                    if c == '\"' {
                        return Ok(TokenType::String(content));
                    }
                    content.push(c);
                }

                Err(TokenTypeParseError::UnterminatedString)
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

                Ok(TokenType::Number(number_string))
            }
            c if c.is_alphabetic() || c == &'_' => {
                let mut identifier = String::from(*c);
                while let Some(char) = chars.next_if(|c| c.is_alphanumeric() || c == &'_') {
                    identifier.push(char);
                }
                let token_type = match identifier.as_str() {
                    "and" => TokenType::And,
                    "class" => TokenType::Class,
                    "else" => TokenType::Else,
                    "false" => TokenType::False,
                    "for" => TokenType::For,
                    "fun" => TokenType::Fun,
                    "if" => TokenType::If,
                    "nil" => TokenType::Nil,
                    "or" => TokenType::Or,
                    "print" => TokenType::Print,
                    "return" => TokenType::Return,
                    "super" => TokenType::Super,
                    "this" => TokenType::This,
                    "true" => TokenType::True,
                    "var" => TokenType::Var,
                    "while" => TokenType::While,
                    _ => TokenType::Identifier(identifier),
                };
                Ok(token_type)
            }
            _ => Err(TokenTypeParseError::UnexpectedCharacter),
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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_string: String;
        let name = match &self {
            TokenType::Eof => "EOF  null",
            TokenType::LeftParen => "LEFT_PAREN ( null",
            TokenType::RightParen => "RIGHT_PAREN ) null",
            TokenType::LeftBrace => "LEFT_BRACE { null",
            TokenType::RightBrace => "RIGHT_BRACE } null",
            TokenType::Comma => "COMMA , null",
            TokenType::Dot => "DOT . null",
            TokenType::Minus => "MINUS - null",
            TokenType::Plus => "PLUS + null",
            TokenType::Semicolon => "SEMICOLON ; null",
            TokenType::Star => "STAR * null",
            TokenType::Bang => "BANG ! null",
            TokenType::BangEqual => "BANG_EQUAL != null",
            TokenType::Equal => "EQUAL = null",
            TokenType::EqualEqual => "EQUAL_EQUAL == null",
            TokenType::Greater => "GREATER > null",
            TokenType::GreaterEqual => "GREATER_EQUAL >= null",
            TokenType::Less => "LESS < null",
            TokenType::LessEqual => "LESS_EQUAL <= null",
            TokenType::Slash => "SLASH / null",
            TokenType::String(s) => {
                temp_string = format!("STRING \"{s}\" {s}");
                temp_string.as_str()
            }
            TokenType::Number(n) => {
                temp_string = format!("NUMBER {n} {}", format_number_string(n));
                temp_string.as_str()
            }
            TokenType::Identifier(i) => {
                temp_string = format!("IDENTIFIER {i} null");
                temp_string.as_str()
            }
            TokenType::And => "AND and null",
            TokenType::Class => "CLASS class null",
            TokenType::Else => "ELSE else null",
            TokenType::False => "FALSE false null",
            TokenType::For => "FOR for null",
            TokenType::Fun => "FUN fun null",
            TokenType::If => "IF if null",
            TokenType::Nil => "NIL nil null",
            TokenType::Or => "OR or null",
            TokenType::Print => "PRINT print null",
            TokenType::Return => "RETURN return null",
            TokenType::Super => "SUPER super null",
            TokenType::This => "THIS this null",
            TokenType::True => "TRUE true null",
            TokenType::Var => "VAR var null",
            TokenType::While => "WHILE while null",
        };
        write!(f, "{}", name)
    }
}

pub struct Token {
    pub token_type: TokenType,
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
    matches!(char, ' ' | '\n' | '\r' | '\t')
}

pub fn tokenize(input: &str, line: usize) -> Vec<Result<Token, String>> {
    let mut token_vec: Vec<Result<Token, String>> = vec![];
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
            Ok(token_type) => token_vec.push(Ok(Token::new(token_type))),
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
