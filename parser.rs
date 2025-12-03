use crate::lexer::Lexer;
use crate::tokens::TCode;
use crate::tokens::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    indent: usize,
}

impl Parser {

    pub fn new(mut lexer: Lexer) -> Self {
        let first = lexer.next_token();
        let mut parser = Self {
            lexer,
            current_token : first,
            indent: 0,
        };
        parser
    }

    pub fn curr(&self) -> &TCode {
        &self.current_token.code
    }

    pub fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn peek(&mut self, symbol: &TCode) -> bool {
        self.curr() == &symbol.clone()
    }

    pub fn expect(&mut self, symbol: TCode) {
        let curr_token = self.curr();

        if curr_token == &symbol {
            println!("{:<indent$}expect({:?})", "", symbol, indent = self.indent);
            self.advance();
        } else {
            panic!(
                "Syntax error: expected {:?}, but found {:?}",
                symbol, curr_token
            );
        }
    }

    pub fn expect_id(&mut self) -> String {
        match self.curr() {
            TCode::ID(name) => {
                let out = name.clone();
                self.advance();
                out
            }
            other => panic!("Expected identifier, found {:?}", other),
        }
    }

    pub fn accept(&mut self, symbol: TCode) -> bool {
        if self.curr() == &symbol {
            self.advance();
            true
        } else {
            false
        }
    }

}