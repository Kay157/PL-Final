use crate::tokens::{TCode, Token, TLoc, TPos};

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    row: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            src: input.chars().collect(),
            pos: 0,
            row: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).cloned()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.src.get(self.pos).cloned();
        if let Some(c) = ch {
            self.pos += 1;
            if c == '\n' {
                self.row += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        ch
    }

    fn make_loc(&self, start_row: usize, start_col: usize, len: usize) -> TLoc {
        TLoc {
            first: TPos::new(start_row, start_col, len),
            last: TPos::new(self.row, self.col - 1, 1),

        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    // FSM states for identifiers/keywords
    fn scan_ident_or_keyword(&mut self) -> Token {
        let sr = self.row;
        let sc = self.col;

        let mut buf = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                buf.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let code = match buf.as_str() {
            "func" => TCode::KW_FUNC,
            "let" => TCode::KW_LET,
            "if" => TCode::KW_IF,
            "else" => TCode::KW_ELSE,
            "while" => TCode::KW_WHILE,
            "return" => TCode::KW_RETURN,
            "print" => TCode::KW_PRINT,
            "true" => TCode::BOOL(true),
            "false" => TCode::BOOL(false),
            _ => TCode::ID(buf),
        };

        let len = self.col - sc;
        Token {
            code,
            loc: self.make_loc(sr, sc, len),
        }
    }

    // FSM for integers
    fn scan_int(&mut self) -> Token {
        let sr = self.row;
        let sc = self.col;

        let mut buf = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                buf.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let value = buf.parse::<i64>().unwrap();
        let len = self.col - sc;
        Token {
            code: TCode::INT(value),
            loc: self.make_loc(sr, sc, len),
        }
    }

    fn two_char_operator(&mut self, expected: char, code: TCode) -> Option<TCode> {
        if let Some(c) = self.peek() {
            if c == expected {
                self.advance();
                return Some(code);
            }
        }
        None
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let sr = self.row;
        let sc = self.col;

        let ch = match self.advance() {
            Some(c) => c,
            None => return Token::from(TCode::EOI),
        };

        // Identifiers and keywords
        if ch.is_ascii_alphabetic() || ch == '_' {
            // backtrack one character
            self.pos -= 1;
            self.col -= 1;
            return self.scan_ident_or_keyword();
        }

        // Integers
        if ch.is_ascii_digit() {
            // backtrack one character
            self.pos -= 1;
            self.col -= 1;
            return self.scan_int();
        }

        // Operators / punctuation
        match ch {
            '=' => {
                if let Some(tok) = self.two_char_operator('=', TCode::OP_EQUAL) {
                    return Token { code: tok, loc: self.make_loc(sr, sc, 2) };
                }
                return Token { code: TCode::OP_ASSIGN, loc: self.make_loc(sr, sc, 1) };
            }

            '!' => {
                if let Some(tok) = self.two_char_operator('=', TCode::OP_NOT_EQUAL) {
                    return Token { code: tok, loc: self.make_loc(sr, sc, 2) };
                }
                return Token { code: TCode::OP_NOT, loc: self.make_loc(sr, sc, 1) };
            }

            '<' => return Token { code: TCode::OP_LT, loc: self.make_loc(sr, sc, 1) },
            '>' => return Token { code: TCode::OP_GT, loc: self.make_loc(sr, sc, 1) },

            '+' => return Token { code: TCode::OP_ADD, loc: self.make_loc(sr, sc, 1) },
            '-' => return Token { code: TCode::OP_SUB, loc: self.make_loc(sr, sc, 1) },
            '*' => return Token { code: TCode::OP_MUL, loc: self.make_loc(sr, sc, 1) },
            '/' => return Token { code: TCode::OP_DIV, loc: self.make_loc(sr, sc, 1) },

            '&' => return Token { code: TCode::OP_AND, loc: self.make_loc(sr, sc, 1) },
            '|' => return Token { code: TCode::OP_OR, loc: self.make_loc(sr, sc, 1) },

            '(' => return Token { code: TCode::PAREN_L, loc: self.make_loc(sr, sc, 1) },
            ')' => return Token { code: TCode::PAREN_R, loc: self.make_loc(sr, sc, 1) },
            '[' => return Token { code: TCode::BRACKET_L, loc: self.make_loc(sr, sc, 1) },
            ']' => return Token { code: TCode::BRACKET_R, loc: self.make_loc(sr, sc, 1) },

            ';' => return Token { code: TCode::SEMICOLON, loc: self.make_loc(sr, sc, 1) },
            ',' => return Token { code: TCode::COMMA, loc: self.make_loc(sr, sc, 1) },

            _ => return Token::error(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut out = Vec::new();

        loop {
            let t = self.next_token();
            if let TCode::EOI = t.code {
                out.push(t);
                break;
            }
            out.push(t);
        }

        out
    }
}

