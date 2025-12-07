use crate::parser::Parser;
use crate::tokens::{TCode, TreeCode};
use crate::mtree::MTree;
use std::cell::RefCell;
use std::rc::Rc;

impl Parser {

    // Program Structure
    // program = { function } ;
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

    // Function Definitions
    // function = "func" ID "(" [ parameters ] ")" block ;
    pub fn parse_func(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_func()");
        self.indent_increment();

        self.expect(TCode::KW_FUNC);

        let func_node = MTree::new(TreeCode::FUNCTION);
        ast_node.borrow_mut()._push(func_node.clone());

        let name = self.expect_id();
        let id_node = MTree::new(TreeCode::IDENTIFIER(name));
        func_node.borrow_mut()._push(id_node);

        self.expect(TCode::PAREN_L);
        self.parse_parameter_list(&func_node);
        self.expect(TCode::PAREN_R);

        self.parse_block_nest(&func_node);

        self.indent_decrement();
    }

    // parameters = ID { "," ID } ;
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

    pub fn parse_parameter(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_parameter()");
        self.indent_increment();

        let param_node = MTree::new(TreeCode::PARAMETER);

        // get the actual name!!
        let name = self.expect_id();
        let id_node = MTree::new(TreeCode::IDENTIFIER(name));

        // attach identifier under parameter node
        param_node.borrow_mut()._push(id_node);

        // attach parameter to list
        ast_node.borrow_mut()._push(param_node);

        self.indent_decrement();
    }


    // Blocks
    // block = "[" { statement } "]" ;
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

    // Statements
    // statement =
    // let_stmt
    // | assign_stmt
    // | if_stmt
    // | while_stmt
    // | return_stmt
    // | print_stmt
    // | expr_stmt
    // ;
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
            TCode::ID(s) => {
                if self.peek_next().code == TCode::OP_ASSIGN {
                        self.parse_assign_statement(&statement_node);
                } else {
                    let expr_node = self.parse_expression();
                    statement_node.borrow_mut()._push(expr_node);
                    self.expect(TCode::SEMICOLON);
                }
            },
            TCode::SEMICOLON => self.advance(),
            other => panic!("Unexpected statement token, found {:?}", other),
        }
        self.indent_decrement();
    }

    // return_stmt = "return" expression ";" ;
    pub fn parse_return_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_return_statement");
        self.indent_increment();

        let return_node = MTree::new(TreeCode::RETURN);

        self.expect(TCode::KW_RETURN);
        let expr_node = self.parse_expression();
        return_node.borrow_mut()._push(expr_node);

        self.expect(TCode::SEMICOLON);
        ast_node.borrow_mut()._push(return_node.clone());

        self.indent_decrement();
    }

    // let_stmt = "let" ID ";" ;
    pub fn parse_let_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_let_statement()");
        self.indent_increment();

        let let_node = MTree::new(TreeCode::LET);
        self.expect(TCode::KW_LET);
        let name = self.expect_id();
        let id_node = MTree::new(TreeCode::IDENTIFIER(name));
        let_node.borrow_mut()._push(id_node);
        if self.curr() == &TCode::OP_ASSIGN {
            self.expect(TCode::OP_ASSIGN);
            let expr_node = self.parse_expression();
            let_node.borrow_mut()._push(expr_node);
        }

        self.expect(TCode::SEMICOLON);
        ast_node.borrow_mut()._push(let_node);
        self.indent_decrement();
    }

    // if_stmt = "if" expression block "else" block ;
    pub fn parse_if_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_if_statement()");
        self.indent_increment();

        let if_node = MTree::new(TreeCode::IF);

        self.expect(TCode::KW_IF);
        let expr_node= self.parse_expression();
        if_node.borrow_mut()._push(expr_node.clone());

        self.parse_block_nest(&if_node);
        if self.accept(TCode::KW_ELSE) {
            self.parse_block_nest(&if_node);
        }

        ast_node.borrow_mut()._push(if_node.clone());
        self.indent_decrement();
    }

    // print_stmt = "print" expression ";" ;
    pub fn parse_print_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_print_statement()");
        self.indent_increment();

        let print_node = MTree::new(TreeCode::PRINT);

        self.expect(TCode::KW_PRINT);
        let expr_node = self.parse_expression();
        print_node.borrow_mut()._push(expr_node.clone());

        self.expect(TCode::SEMICOLON);
        ast_node.borrow_mut()._push(print_node.clone());
        self.indent_decrement();
    }

    // while_stmt = "while" expression block ;
    pub fn parse_while_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_while_statement()");
        self.indent_increment();

        let while_node = MTree::new(TreeCode::WHILE);

        self.expect(TCode::KW_WHILE);
        let expr_node = self.parse_expression();
        while_node.borrow_mut()._push(expr_node.clone());

        ast_node.borrow_mut()._push(while_node.clone());
        self.indent_decrement();
    }

    // assign_stmt = ID "=" expression ";" ;
    pub fn parse_assign_statement(&mut self, ast_node: &Rc<RefCell<MTree>>) {
        self.indent_print("parse_assign_statement()");
        self.indent_increment();

        let assign_node = MTree::new(TreeCode::ASSIGN);
        ast_node.borrow_mut()._push(assign_node.clone());

        if let TCode::ID(name) = self.curr() {
            let id_node = MTree::new(TreeCode::IDENTIFIER(name.clone()));
            assign_node.borrow_mut()._push(id_node);
            self.advance();
        } else {
            panic!("Expected identifier at assignment start");
        }

        self.expect(TCode::OP_ASSIGN);

        let expr_node = self.parse_expression();
        assign_node.borrow_mut()._push(expr_node);

        self.expect(TCode::SEMICOLON);

        self.indent_decrement();
    }


    // expr_stmt = expression ";" ;
    pub fn parse_expression(&mut self) -> Rc<RefCell<MTree>> {
        self.parse_logic_or()
    }

    // logic_or = logic_and { "|" logic_and } ;
    fn parse_logic_or(&mut self) -> Rc<RefCell<MTree>> {
        let mut left = self.parse_logic_and();
        while self.curr() == &TCode::OP_OR {
            let op = self.curr().clone();
            self.advance();
            let right = self.parse_logic_and();
            left = self.make_binary_op_node(op, left, right);
        }
        left
    }

    // logic_and = equality { "&" equality } ;
    fn parse_logic_and(&mut self) -> Rc<RefCell<MTree>> {
        let mut left = self.parse_equality();
        while self.curr() == &TCode::OP_AND {
            let op = self.curr().clone();
            self.advance();
            let right = self.parse_equality();
            left = self.make_binary_op_node(op, left, right);
        }
        left
    }

    // equality = relational { ( "==" | "!=" ) relational } ;
    fn parse_equality(&mut self) -> Rc<RefCell<MTree>> {
        let mut left = self.parse_relational();
        while self.curr() == &TCode::OP_EQUAL || self.curr() == &TCode::OP_NOT_EQUAL {
            let op = self.curr().clone();
            self.advance();
            let right = self.parse_relational();
            left = self.make_binary_op_node(op, left, right);
        }
        left
    }

    // relational = additive { ( "<" | ">" ) additive } ;
    fn parse_relational(&mut self) -> Rc<RefCell<MTree>> {
        let mut left = self.parse_additive();
        while self.curr() == &TCode::OP_LT || self.curr() == &TCode::OP_GT {
            let op = self.curr().clone();
            self.advance();
            let right = self.parse_additive();
            left = self.make_binary_op_node(op, left, right);
        }
        left
    }

    // additive = multiplicative { ( "+" | "-" ) multiplicative } ;
    fn parse_additive(&mut self) -> Rc<RefCell<MTree>> {
        let mut left = self.parse_multiplicative();
        while self.curr() == &TCode::OP_ADD || self.curr() == &TCode::OP_SUB {
            let op = self.curr().clone();
            self.advance();
            let right = self.parse_multiplicative();
            left = self.make_binary_op_node(op, left, right);
        }
        left
    }

    // multiplicative = unary { ( "*" | "/" ) unary } ;
    fn parse_multiplicative(&mut self) -> Rc<RefCell<MTree>> {
        let mut left = self.parse_unary();
        while self.curr() == &TCode::OP_MUL || self.curr() == &TCode::OP_DIV {
            let op = self.curr().clone();
            self.advance();
            let right = self.parse_unary();
            left = self.make_binary_op_node(op, left, right);
        }
        left
    }

    // unary = ( "!" | "-" ) unary | primary ;
    fn parse_unary(&mut self) -> Rc<RefCell<MTree>> {
        match self.curr() {
            TCode::OP_NOT | TCode::OP_SUB => {
                let op = self.curr().clone();
                self.advance();
                let expr = self.parse_unary();
                let node = MTree::new(TreeCode::OPERATOR(format!("{:?}", op)));
                node.borrow_mut()._push(expr);
                node
            }
            _ => self.parse_primary(),
        }
    }

    // primary =
    // INT
    // | BOOL
    // | ID
    // | function_call
    // | "(" expression ")"
    fn parse_primary(&mut self) -> Rc<RefCell<MTree>> {
        let current_token = self.curr().clone();

        match current_token {
            TCode::INT(val) => {
                let node = MTree::new(TreeCode::INT_LITERAL(val.clone()));
                self.advance();
                node
            }
            TCode::BOOL(val) => {
                let node = MTree::new(TreeCode::BOOL_LITERAL(val.clone()));
                self.advance();
                node
            }
            TCode::ID(name) => {
                let id_node = MTree::new(TreeCode::IDENTIFIER(name.clone()));
                self.advance();
                if self.curr() == &TCode::PAREN_L {
                    self.advance();
                    let func_call_node = MTree::new(TreeCode::FUNCTION_CALL(name.clone()));
                    func_call_node.borrow_mut()._push(id_node.clone());
                    if self.curr() != &TCode::PAREN_R {
                        loop {
                            let arg = self.parse_expression();
                            func_call_node.borrow_mut()._push(arg);
                            if !self.accept(TCode::COMMA) {
                                break;
                            }
                        }
                    }
                    self.expect(TCode::PAREN_R);
                    func_call_node
                } else {
                    id_node
                }
            }
            TCode::PAREN_L => {
                self.advance();
                let expr = self.parse_expression();
                self.expect(TCode::PAREN_R);
                expr
            }
            other => panic!("Unexpected token in expression: {:?}", other),
        }
    }

    fn make_binary_op_node(&self, op: TCode, left: Rc<RefCell<MTree>>, right: Rc<RefCell<MTree>>) -> Rc<RefCell<MTree>> {
        let op_node = MTree::new(TreeCode::OPERATOR(format!("{:?}", op)));
        op_node.borrow_mut()._push(left);
        op_node.borrow_mut()._push(right);
        op_node
    }
}