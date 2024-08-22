use std::fmt::Display;

use crate::scanner::TokenType;

pub struct Parser {
    tokens: Vec<TokenType>,
}

impl Parser {
    fn parse(&self) -> Vec<Result<Expr, String>> {
        let mut res: Vec<Result<Expr, String>> = Vec::new();
        for token in &self.tokens {
            res.push(primary(token));
        }
        res
    }
}

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
enum Object {
    String(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_str: String;

        let name = match self {
            Object::String(s) => {
                temp_str = format!("{s}");
                &temp_str
            }
        };
        write!(f, "{name}")
    }
}
