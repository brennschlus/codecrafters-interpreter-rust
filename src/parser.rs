use std::{fmt::Display, iter::Peekable};

use crate::scanner::Token;

pub fn parse_tokens<T>(token_iter: &mut Peekable<T>) -> Vec<Result<Expr, String>>
where
    T: Iterator<Item = Result<Token, String>>,
{
    let mut exprs: Vec<Result<Expr, String>> = Vec::new();

    while let Some(token) = token_iter.next() {
        exprs.push(token.and_then(|token| Expr::from_tokens(token_iter, token)));
    }
    exprs
}
pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        token: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Call {
        calee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        obj: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl Expr {
    fn from_tokens<T>(token_iter: &mut Peekable<T>, token: Token) -> Result<Expr, String>
    where
        T: Iterator<Item = Result<Token, String>>,
    {
        match token {
            Token::True => Ok(Expr::Literal {
                value: Object::String("true".to_owned()),
            }),
            Token::False => Ok(Expr::Literal {
                value: Object::String("false".to_owned()),
            }),
            Token::Nil => Ok(Expr::Literal {
                value: Object::String("nil".to_owned()),
            }),
            Token::Number(number) => {
                let parsed_number = number.parse::<f64>().unwrap_or(0.0);
                Ok(Expr::Literal {
                    value: Object::Number(parsed_number),
                })
            }
            Token::String(s) => Ok(Expr::Literal {
                value: Object::String(s.to_string()),
            }),
            Token::LeftParen => {
                let next_expr = {
                    if let Some(next_token) = token_iter.next() {
                        match next_token {
                            Ok(token) => Expr::from_tokens(token_iter, token),
                            Err(e) => Err(e),
                        }
                    } else {
                        Err("Unterminated paren".to_owned())
                    }
                };
                match token_iter.next() {
                    Some(Ok(Token::RightParen)) => Ok(Expr::Grouping {
                        //TODO(me): correct path when next_expr is Err
                        expression: Box::new(next_expr.unwrap_or(Expr::Literal {
                            value: Object::String("nil".to_owned()),
                        })),
                    }),
                    _ => Err("pare".to_owned()),
                }
            }
            _ => Err("Wrong expression".to_owned()),
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_str: String;
        let name = match self {
            Expr::Literal { value } => {
                temp_str = format!("{value}");
                &temp_str
            }
            Expr::Grouping { expression } => {
                temp_str = format!("(group {expression})");
                &temp_str
            }
            _ => "Wrong expr",
        };
        write!(f, "{}", name)
    }
}
pub enum Object {
    String(String),
    Number(f64),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_str: String;
        let name = match self {
            Object::String(s) => &s,
            Object::Number(number) => {
                temp_str = format!("{number:?}");
                &temp_str
            }
        };
        write!(f, "{name}")
    }
}
