use std::fmt::Display;

use crate::scanner::TokenType;

pub fn primary(token: &TokenType) -> Result<Expr, String> {
    match token {
        TokenType::True => Ok(Expr::Literal {
            value: Object::String("true".to_owned()),
        }),
        TokenType::False => Ok(Expr::Literal {
            value: Object::String("false".to_owned()),
        }),
        TokenType::Nil => Ok(Expr::Literal {
            value: Object::String("nil".to_owned()),
        }),
        TokenType::Number(number) => {
            let parsed_number = number.parse::<f64>().unwrap_or(0.0);
            Ok(Expr::Literal {
                value: Object::Number(parsed_number),
            })
        }
        TokenType::String(s) => Ok(Expr::Literal {
            value: Object::String(s.to_string()),
        }),
        _ => Err("Wrong expression".to_owned()),
    }
}
pub enum Expr {
    Binary {
        left: Box<Expr>,
        right: Box<Expr>,
        token: TokenType,
    },
    Assign {
        name: TokenType,
        value: Box<Expr>,
    },
    Call {
        calee: Box<Expr>,
        paren: TokenType,
        arguments: Vec<Expr>,
    },
    Get {
        obj: Box<Expr>,
        name: TokenType,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Expr>,
        operator: TokenType,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: TokenType,
        value: Box<Expr>,
    },
    Super {
        keyword: TokenType,
        method: TokenType,
    },
    This {
        keyword: TokenType,
    },
    Unary {
        operator: TokenType,
        right: Box<Expr>,
    },
    Variable {
        name: TokenType,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_str: String;
        let name = match self {
            Expr::Literal { value } => {
                temp_str = format!("{value}");
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
            Object::String(s) => s,
            Object::Number(number) => {
                temp_str = format!("{number:?}");
                &temp_str
            }
        };
        write!(f, "{name}")
    }
}
