#![allow(non_camel_case_types)]
#![allow(dead_code)]

use std::fmt;
use std::mem::discriminant;

#[derive(Clone)]
pub enum TCode {

    // General
    EOI,
    ERROR,

    // Literals and Identifiers
    ID(String),
    INT(i64),
    BOOL(bool),

    // Keywords
    KW_FUNC,
    KW_LET,
    KW_IF,
    KW_ELSE,
    KW_WHILE,
    KW_RETURN,
    KW_PRINT,

    // Arithmetic Operators
    OP_ASSIGN,
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,

    // Relational Operators
    OP_EQUAL,
    OP_NOT_EQUAL,
    OP_LT,
    OP_GT,

    // Logical Operators
    OP_AND,
    OP_OR,
    OP_NOT,

    // Nesting
    PAREN_L,
    PAREN_R,
    BRACKET_L,
    BRACKET_R,

    // Separators
    SEMICOLON,
    COMMA,
}

impl fmt::Debug for TCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TCode::EOI => write!(f, "EOI"),
            TCode::ERROR => write!(f, "ERROR"),

            TCode::ID(name) => write!(f, "ID(\"{}\")", name),
            TCode::INT(value) => write!(f, "INT({})", value),
            TCode::BOOL(value) => write!(f, "BOOL({})", value),

            TCode::OP_ASSIGN => write!(f, "="),
            TCode::OP_ADD => write!(f, "+"),
            TCode::OP_SUB => write!(f, "-"),
            TCode::OP_MUL => write!(f, "*"),
            TCode::OP_DIV => write!(f, "/"),

            TCode::OP_LT => write!(f, "<"),
            TCode::OP_GT => write!(f, ">"),
            TCode::OP_EQUAL => write!(f, "=="),
            TCode::OP_NOT_EQUAL => write!(f, "!="),
            //TCode::OP_LE => write!(f, "<="),
            //TCode::OP_GE => write!(f, ">="),

            TCode::OP_AND => write!(f, "&"),
            TCode::OP_OR => write!(f, "|"),
            TCode::OP_NOT => write!(f, "!"),

            TCode::PAREN_L => write!(f, "("),
            TCode::PAREN_R => write!(f, ")"),
            TCode::BRACKET_L => write!(f, "["),
            TCode::BRACKET_R => write!(f, "]"),

            TCode::COMMA => write!(f, ","),
            TCode::SEMICOLON => write!(f, ";"),

            TCode::KW_FUNC => write!(f, "FUNC"),
            TCode::KW_LET => write!(f, "LET"),
            TCode::KW_IF => write!(f, "IF"),
            TCode::KW_ELSE => write!(f, "ELSE"),
            TCode::KW_WHILE => write!(f, "WHILE"),
            TCode::KW_RETURN => write!(f, "RETURN"),
            TCode::KW_PRINT => write!(f, "PRINT"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TPos {
    pub row: usize,
    pub col: usize,
    pub len: usize,
}

impl TPos {
    pub fn new(row: usize, col: usize, len: usize) -> Self {
        Self { row, col, len }
    }
}

#[derive(Debug, Clone)]
pub struct TLoc {
    pub first: TPos,
    pub last: TPos,
}

impl TLoc {
    pub fn empty() -> TLoc {
        TLoc {
            first: TPos::new(0, 0, 0),
            last: TPos::new(0, 0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub code: TCode,
    pub loc: TLoc,
}

impl Token {
    pub fn from(code: TCode) -> Token {
        Token { code, loc: TLoc::empty() }
    }

    pub fn error() -> Token {
        Token { code: TCode::ERROR, loc: TLoc::empty() }
    }

    pub fn id(name: &str) -> Token {
        Token { code: TCode::ID(name.to_string()), loc: TLoc::empty() }
    }
}

impl PartialEq for TCode {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for TCode {}