use crate::parser::Parser;
use crate::tokens::{TCode, Token, TreeCode};
use crate::mtree::MTree;
use std::cell::RefCell;
use std::rc::Rc;

// for return statements should we add them
/*pub enum TypeOrIdent {
    Type(TCode),
    Ident(String),
}*/

impl Parser {
    // parse: Function* EOF
    pub fn parse(&mut self) -> Rc<RefCell<MTree>> {
        let root = MTree::new(TreeCode::PROGRAM);
        loop {
            match self.curr() {
                TCode::KW_FUNC => {
                    self.parse_func(&root);
                }
                TCode::EOI => break,
                other => panic!(
                    "Expected function declaration or end of input, found {:?}",
                    other
                ),
            }
        }
        root
    }

    // parse_func: "func" ID "(" ParameterList ")" TypeOrIdent ] BlockNest
    pub fn parse_func(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_func()");
        self.indent_increment();

        self.expect(TCode::KW_FUNC);
        let func_node = MTree::new(TreeCode::FUNCTION);
        ast_node.borrow_mut()._push(func_node.clone());
        self.expect_id();

        self.expect(TCode::PAREN_L);
        self.parse_parameter_list(&func_node);
        self.expect(TCode::PAREN_R);

        self.parse_block_nest(&func_node);

        self.indent_decrement();
    }


    // parse_parameter_list: "(" [ Parameter { "," Parameter } ] ")"
    pub fn parse_parameter_list(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_parameter_list()");
        self.indent_increment();

        let param_list_node = MTree::new(TreeCode::PARAM_LIST);
        ast_node.borrow_mut()._push(param_list_node.clone());

        if self.curr() == &TCode::PAREN_R {
            return;
        }
        self.parse_parameter(&param_list_node);
        while self.accept(TCode::COMMA) {
            self.parse_parameter(&param_list_node);
        }
        self.indent_decrement();
    }

    // parse_parameter: ID
    pub fn parse_parameter(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_parameter()");
        self.indent_increment();

        let param_node = MTree::new(TreeCode::PARAMETER);
        ast_node.borrow_mut()._push(param_node.clone());
        self.expect_id();

        self.indent_decrement();
    }

    // parse_block_nest: "[" { Statement } "]"
    pub fn parse_block_nest(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_block_nest()");
        self.indent_increment();

        let block_node = MTree::new(TreeCode::BLOCK);
        ast_node.borrow_mut()._push(block_node.clone());

        self.expect(TCode::BRACKET_L);
        while !self.accept(TCode::BRACKET_R) {
            self.parse_statement(&block_node);
        }
        self.indent_decrement();
    }

    // parse_statement: RETURN | LET | IF | "{"
    pub fn parse_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_statement()");
        self.indent_increment();

        let statement_node = MTree::new(TreeCode::STATEMENT);
        ast_node.borrow_mut()._push(statement_node.clone());

        match self.curr() {
            TCode::KW_RETURN => self.parse_return_statement(&statement_node),
            TCode::KW_LET => self.parse_let_statement(&statement_node),
            TCode::KW_IF => self.parse_if_statement(&statement_node),
            TCode::KW_PRINT => self.parse_print_statement(&statement_node),
            TCode::KW_WHILE => self.parse_while_statement(&statement_node),
            TCode::BRACKET_L => self.parse_block_nest(&statement_node),
            TCode::ID(s) => self.parse_expression(),
            TCode::SEMICOLON => self.advance(),
            other => panic!("Unexpected statement token, found {:?}", other),
        }
        self.indent_decrement();
    }

    // parse_return_statement: "return" (ID | Literal) ";"
    pub fn parse_return_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_return_statement");
        self.indent_increment();

        let return_node = MTree::new(TreeCode::RETURN);
        ast_node.borrow_mut()._push(return_node.clone());

        self.expect(TCode::KW_RETURN);
        self.parse_expression();

        self.indent_decrement();
    }

    // parse_let_statement: "let" ID ":" TypeOrIdent "=" Expression ";"
    pub fn parse_let_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_let_statement()");
        self.indent_increment();

        let let_node = MTree::new(TreeCode::LET);
        ast_node.borrow_mut()._push(let_node.clone());

        self.expect(TCode::KW_LET);
        self.expect_id();
        if self.curr() == &TCode::OP_ASSIGN {
            self.parse_expression();
        }
        self.indent_decrement();
    }

    // parse_if_statement: "if" Expression BlockNest [ "else" BlockNest ]
    pub fn parse_if_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_if_statement()");
        self.indent_increment();

        let if_node = MTree::new(TreeCode::IF);
        ast_node.borrow_mut()._push(if_node.clone());

        self.expect(TCode::KW_IF);
        self.parse_expression();
        self.parse_block_nest(&if_node);
        if self.accept(TCode::KW_ELSE) {
            self.parse_block_nest(&if_node);
        }
        self.indent_decrement();
    }

    // parse_print_statement: "print" expression;
    pub fn parse_print_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_print_statement()");
        self.indent_increment();

        let print_node = MTree::new(TreeCode::PRINT);
        ast_node.borrow_mut()._push(print_node.clone());

        self.expect(TCode::KW_PRINT);
        self.parse_expression();
        self.indent_decrement();
    }

    // parse_while_statement: "while" Expression BlockNest
    pub fn parse_while_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_while_statement()");
        self.indent_increment();

        let while_node = MTree::new(TreeCode::WHILE);
        ast_node.borrow_mut()._push(while_node.clone());

        self.expect(TCode::KW_WHILE);
        self.parse_expression();
        self.indent_decrement();
    }

    // parse_expression: ID | Literal | "true" | "false"
    pub fn parse_expression(&mut self) {

        while self.curr() != &TCode::BRACKET_L && self.curr() != &TCode::SEMICOLON && self.curr() != &TCode::EOI{
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
                TCode::OP_ADD => {
                    self.advance();
                }
                TCode::OP_SUB => {
                    self.advance();
                }
                TCode::OP_MUL => {
                    self.advance();
                }
                TCode::OP_DIV => {
                    self.advance();
                }
                TCode::OP_LT => {
                    self.advance();
                }
                TCode::OP_GT => {
                    self.advance();
                }
                TCode::OP_EQUAL => {
                    self.advance();
                }
                TCode::OP_NOT_EQUAL => {
                    self.advance();
                }
                TCode::OP_AND => {
                    self.advance();
                }
                TCode::OP_OR => {
                    self.advance();
                }
                TCode::OP_NOT => {
                    self.advance();
                }
                TCode::OP_ASSIGN => {
                    self.advance();
                }
                TCode::PAREN_L => {
                    self.advance();
                    while self.curr() != &TCode::PAREN_R {
                        self.advance();
                    }
                }
                TCode::PAREN_R => {
                    self.advance();
                }
                other => panic!("Expected expression, found {:?}", other),
            }
        }
        match self.curr() {
            TCode::BRACKET_R => self.expect(TCode::BRACKET_R),
            TCode::SEMICOLON => self.expect(TCode::SEMICOLON),
            _ => return,
        }
    }
}