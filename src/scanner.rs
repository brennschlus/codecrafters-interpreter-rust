use std::fmt::Display;
use std::iter::Peekable;

#[derive(Clone, PartialEq, Eq)]
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
            '!' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Ok(Token::BangEqual)
                } else {
                    Ok(Token::Bang)
                }
            }
            '=' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Ok(Token::EqualEqual)
                } else {
                    Ok(Token::Equal)
                }
            }
            '>' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Ok(Token::GreaterEqual)
                } else {
                    Ok(Token::Greater)
                }
            }
            '<' => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    Ok(Token::LessEqual)
                } else {
                    Ok(Token::Less)
                }
            }
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
    pub fn raw_symbol(&self) -> String {
        match &self {
            Token::Bang => "!".to_owned(),
            Token::Minus => "-".to_owned(),
            _ => "nill".to_owned(),
        }
    }
}
pub fn format_number_string(string: &str) -> String {
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
        match &self {
            Token::Eof => write!(f, "EOF  null"),
            Token::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Token::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Token::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Token::RightBrace => write!(f, "RIGHT_BRACE }} null"),
            Token::Comma => write!(f, "COMMA , null"),
            Token::Dot => write!(f, "DOT . null"),
            Token::Minus => write!(f, "MINUS - null"),
            Token::Plus => write!(f, "PLUS + null"),
            Token::Semicolon => write!(f, "SEMICOLON ; null"),
            Token::Star => write!(f, "STAR * null"),
            Token::Bang => write!(f, "BANG ! null"),
            Token::BangEqual => write!(f, "BANG_EQUAL != null"),
            Token::Equal => write!(f, "EQUAL = null"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
            Token::Greater => write!(f, "GREATER > null"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL >= null"),
            Token::Less => write!(f, "LESS < null"),
            Token::LessEqual => write!(f, "LESS_EQUAL <= null"),
            Token::Slash => write!(f, "SLASH / null"),
            Token::String(s) => write!(f, "STRING \"{s}\" {s}"),
            Token::Number(n) => write!(f, "NUMBER {n} {}", format_number_string(&n)),
            Token::Identifier(i) => write!(f, "IDENTIFIER {i} null"),
            Token::And => write!(f, "AND and null"),
            Token::Class => write!(f, "CLASS class null"),
            Token::Else => write!(f, "ELSE else null"),
            Token::False => write!(f, "FALSE false null"),
            Token::For => write!(f, "FOR for null"),
            Token::Fun => write!(f, "FUN fun null"),
            Token::If => write!(f, "IF if null"),
            Token::Nil => write!(f, "NIL nil null"),
            Token::Or => write!(f, "OR or null"),
            Token::Print => write!(f, "PRINT print null"),
            Token::Return => write!(f, "RETURN return null"),
            Token::Super => write!(f, "SUPER super null"),
            Token::This => write!(f, "THIS this null"),
            Token::True => write!(f, "TRUE true null"),
            Token::Var => write!(f, "VAR var null"),
            Token::While => write!(f, "WHILE while null"),
        }
    }
}

fn skip_char(char: char) -> bool {
    matches!(char, ' ' | '\n' | '\r' | '\t')
}

pub fn tokenize(input: &str, line: usize) -> Vec<Result<Token, String>> {
    let mut token_vec = Vec::new();
    let mut iter = input.chars().peekable();
    while let Some(char) = iter.next() {
        if skip_char(char) {
            continue;
        }
        if char == '/' && iter.peek().is_some_and(|c| c == &'/') {
            break;
        }
        let token = Token::from_chars(&char, &mut iter);
        token_vec.push(token.map_err(|e| match e {
            TokenParseError::UnexpectedCharacter => {
                format!("[line {}] Error: Unexpected character: {}", line, char)
            }
            TokenParseError::UnterminatedString => {
                format!("[line {line}] Error: Unterminated string.")
            }
        }));
    }

    token_vec
}
