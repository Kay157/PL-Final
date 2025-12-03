use crate::parser::Parser;
use crate::tokens::TCode;

// for return statements should we add them
/*pub enum TypeOrIdent {
    Type(TCode),
    Ident(String),
}*/

impl Parser {
    // parse: Function* EOF
    pub fn parse(&mut self) {
        loop {
            match self.curr() {
                TCode::KW_FUNC => {
                    self.parse_func();
                }
                TCode::EOI => break,
                other => panic!(
                    "Expected function declaration or end of input, found {:?}",
                    other
                ),
            }
        }
    }

    // parse_func: "func" ID "(" ParameterList ")" TypeOrIdent ] BlockNest
    pub fn parse_func(&mut self) {
        self.expect(TCode::KW_FUNC);

        self.expect_id();

        self.expect(TCode::PAREN_L);
        self.parse_parameter_list();
        self.expect(TCode::PAREN_R);

        self.parse_block_nest();
    }


    // parse_parameter_list: "(" [ Parameter { "," Parameter } ] ")"
    pub fn parse_parameter_list(&mut self) {
        if self.curr() == &TCode::PAREN_R {
            return;
        }
        self.parse_parameter();
        while self.accept(TCode::COMMA) {
            self.parse_parameter();
        }
    }

    // parse_parameter: ID
    pub fn parse_parameter(&mut self) {
        self.expect_id();
    }

    // parse_block_nest: "[" { Statement } "]"
    pub fn parse_block_nest(&mut self) {
        self.expect(TCode::BRACKET_L);
        while !self.accept(TCode::BRACKET_R) {
            self.parse_statement();
        }
    }

    // parse_statement: RETURN | LET | IF | "{"
    pub fn parse_statement(&mut self) {
        match self.curr() {
            TCode::KW_RETURN => self.parse_return_statement(),
            TCode::KW_LET => self.parse_let_statement(),
            TCode::KW_IF => self.parse_if_statement(),
            TCode::KW_PRINT => self.parse_print_statement(),
            TCode::KW_WHILE => self.parse_while_statement(),
            TCode::BRACKET_L => self.parse_block_nest(),
            other => panic!("Unexpected statement token, found {:?}", other),
        }
    }

    // parse_return_statement: "return" (ID | Literal) ";"
    pub fn parse_return_statement(&mut self) {
        self.expect(TCode::KW_RETURN);

        match self.curr() {
            TCode::ID(name) => {
                self.advance();
            }
            TCode::INT(s) => {
                self.advance();
            }
            TCode::BOOL(s) => {
                self.advance();
            }
            other => panic!("Expected return statement, found {:?}", other),
        }

        self.expect(TCode::SEMICOLON);
    }

    // parse_let_statement: "let" ID ":" TypeOrIdent "=" Expression ";"
    pub fn parse_let_statement(&mut self) {
        self.expect(TCode::KW_LET);
        self.expect_id();
        self.expect(TCode::OP_ASSIGN);
        self.parse_expression();
        self.expect(TCode::SEMICOLON);

    }

    // parse_if_statement: "if" Expression BlockNest [ "else" BlockNest ]
    pub fn parse_if_statement(&mut self) {
        self.expect(TCode::KW_IF);
        self.parse_expression();
        self.parse_block_nest();
        if self.accept(TCode::KW_ELSE) {
            self.parse_block_nest();
        }
    }

    // parse_print_statement: "print" expression;
    pub fn parse_print_statement(&mut self) {
        self.expect(TCode::KW_PRINT);
        self.parse_expression();
        self.expect(TCode::SEMICOLON);
    }

    // parse_while_statement: "while" Expression BlockNest
    pub fn parse_while_statement(&mut self) {
        self.expect(TCode::KW_WHILE);
        self.parse_block_nest();
    }

    // parse_expression: ID | Literal | "true" | "false"
    pub fn parse_expression(&mut self) {
        match self.curr() {
            TCode::ID(name) => {
                self.advance();
            }
            TCode::INT(s) => {
                self.advance();
            }
            TCode::BOOL(s) => {
                self.advance();
            }
            other => panic!("Expected expression, found {:?}", other),
        }
    }
}