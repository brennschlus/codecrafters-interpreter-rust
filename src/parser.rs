use crate::scanner::TokenType;

pub struct Parser {
    tokens: Vec<TokenType>,
}

enum Expr {
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

enum Object {}
