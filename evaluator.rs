use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

use crate::mtree::MTree;
use crate::tokens::TreeCode;

#[derive(Clone)]
pub enum Value {
    INT(i64),
    FUNC(Rc<RefCell<MTree>>),
    VOID,
}

pub struct Frame {
    variables: HashMap<String, Value>,
    declared: HashSet<String>,
    parent: Option<Rc<RefCell<Frame>>>,
}

impl Frame {
    pub fn new(parent: Option<Rc<RefCell<Frame>>>) -> Self {
        Frame {
            variables: HashMap::new(),
            declared: HashSet::new(),
            parent
        }
    }

    pub fn declare(&mut self, name: &str, val: Value) {
        self.declared.insert(name.to_string());
        self.variables.insert(name.to_string(), val);
    }
    pub fn set(&mut self, name: &str, val: Value) {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), val);
        } else if let Some(ref parent) = self.parent {
            parent.borrow_mut().set(name, val);
        } else {
            // Clean error message and stop
            eprintln!("Runtime Error: variable `{}` used before declaration", name);
            std::process::exit(1);
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if self.variables.contains_key(name) {
            Some(self.variables[name].clone())
        } else if let Some(ref p) = self.parent {
            p.borrow().get(name)
        } else {
            None
        }
    }
}

pub struct Runtime {
    pub functions: HashMap<String, Rc<RefCell<MTree>>>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime { functions: HashMap::new() }
    }

    pub fn run_program(&mut self, program: Rc<RefCell<MTree>>) {

        self.collect_functions(program.clone());

        let main_func = match self.functions.get("main") {
            Some(func) => func.clone(),
            None => {
                eprintln!("Runtime Error: no `main` function found in the program.");
                std::process::exit(1);
            }
        };

        self.call_function(main_func, vec![], None);
    }

    fn collect_functions(&mut self, node: Rc<RefCell<MTree>>) {
        if let TreeCode::FUNCTION = &node.borrow().token {
            if let Some(name_node) = node.borrow().children.get(0) {
                if let TreeCode::IDENTIFIER(name) = &name_node.borrow().token {
                    self.functions.insert(name.clone(), node.clone());
                }
            }
        }
        for child in &node.borrow().children {
            self.collect_functions(child.clone());
        }
    }

    pub fn call_function(
        &self,
        func_node: Rc<RefCell<MTree>>,
        args: Vec<Value>,
        parent_frame: Option<Rc<RefCell<Frame>>>
    ) -> Value {

        let frame = Rc::new(RefCell::new(Frame::new(parent_frame)));

        if let Some(param_list) = func_node.borrow().children.get(1) {
            for (i, p) in param_list.borrow().children.iter().enumerate() {
                if let TreeCode::PARAMETER = &p.borrow().token {
                    if let Some(id) = p.borrow().children.get(0) {
                        if let TreeCode::IDENTIFIER(name) = &id.borrow().token {
                            frame.borrow_mut().variables.insert(name.clone(), args[i].clone());
                        }
                    }
                }
            }
        }

        let block = func_node.borrow().children.get(2).unwrap().clone();
        self.run_block(block, frame).unwrap_or(Value::VOID)
    }

    fn run_block(&self, block: Rc<RefCell<MTree>>, frame: Rc<RefCell<Frame>>) -> Option<Value> {
        for stmt in &block.borrow().children {
            if let Some(val) = self.run_stmt(stmt.clone(), frame.clone()) {
                return Some(val);
            }
        }
        None
    }

    fn run_stmt(&self, stmt: Rc<RefCell<MTree>>, frame: Rc<RefCell<Frame>>) -> Option<Value> {
        match &stmt.borrow().token {

            TreeCode::STATEMENT => {
                if let Some(real_stmt) = stmt.borrow().children.get(0) {
                    return self.run_stmt(real_stmt.clone(), frame);
                }
                None
            }
            TreeCode::BLOCK => self.run_block(stmt.clone(), frame),
            TreeCode::LET => {
                if let Some(id) = stmt.borrow().children.get(0) {
                    if let TreeCode::IDENTIFIER(name) = &id.borrow().token {
                        frame.borrow_mut().variables.insert(name.clone(), Value::INT(0));
                    }
                }
                None
            }
            TreeCode::ASSIGN => {
                let stmt_borrow = stmt.borrow();
                let id = stmt_borrow.children.get(0).unwrap();
                let expr = stmt_borrow.children.get(1).unwrap();
                let val = self.eval_expr(expr.clone(), frame.clone());
                if let TreeCode::IDENTIFIER(name) = &id.borrow().token {
                    frame.borrow_mut().set(name, val);
                }
                None
            }
            TreeCode::PRINT => {
                let stmt_borrow = stmt.borrow();
                let expr = stmt_borrow.children.get(0).unwrap();
                let val = self.eval_expr(expr.clone(), frame.clone());
                match val {
                    Value::INT(i) => println!("{}", i),
                    Value::VOID => println!("void"),
                    _ => println!("unknown"),
                }
                None
            }
            TreeCode::RETURN => {
                let stmt_borrow = stmt.borrow();
                let expr = stmt_borrow.children.get(0).unwrap();
                Some(self.eval_expr(expr.clone(), frame.clone()))
            }
            TreeCode::IF => {
                let stmt_borrow = stmt.borrow();
                let cond_node = stmt_borrow.children.get(0)
                    .expect("IF missing condition");
                let cond_val = self.eval_expr(cond_node.clone(), frame.clone());
                let branch_idx = if let Value::INT(i) = cond_val { i != 0 } else { false };
                let block_node = stmt_borrow.children.get(if branch_idx { 1 } else { 2 })
                    .expect("IF missing branch");
                let block = self.unwrap_block(block_node.clone());
                self.run_block(block, frame)
            }
            TreeCode::WHILE => {
                let stmt_borrow = stmt.borrow();
                let cond = stmt_borrow.children.get(0).expect("WHILE missing condition");

                let block = if let Some(body_node) = stmt_borrow.children.get(1) {

                    match body_node.borrow().token {
                        TreeCode::BLOCK | TreeCode::STATEMENT => self.unwrap_block(body_node.clone()),
                        _ => {

                            let b = MTree::new(TreeCode::BLOCK);
                            b.borrow_mut().children.push(body_node.clone());
                            b
                        }
                    }
                } else {

                    let b = MTree::new(TreeCode::BLOCK);
                    b.borrow_mut().children.push(stmt.clone());
                    b
                };

                loop {
                    let cond_val = self.eval_expr(cond.clone(), frame.clone());
                    if let Value::INT(i) = cond_val {
                        if i == 0 { break; }
                    } else {
                        panic!("WHILE condition must evaluate to INT");
                    }

                    if let Some(ret_val) = self.run_block(block.clone(), frame.clone()) {
                        return Some(ret_val);
                    }
                }
                None
            }
            _ => None
        }
    }

    fn unwrap_block(&self, stmt: Rc<RefCell<MTree>>) -> Rc<RefCell<MTree>> {
        match stmt.borrow().token {
            TreeCode::BLOCK => stmt.clone(),
            TreeCode::STATEMENT => {
                if let Some(child) = stmt.borrow().children.get(0) {
                    self.unwrap_block(child.clone())
                } else {
                    panic!("STATEMENT node has no children")
                }
            }
            _ => panic!("Expected BLOCK or STATEMENT, found {:?}", stmt.borrow().token)
        }
    }



    fn eval_expr(&self, expr: Rc<RefCell<MTree>>, frame: Rc<RefCell<Frame>>) -> Value {
        match &expr.borrow().token {
            TreeCode::INT_LITERAL(i) => Value::INT(*i),
            TreeCode::IDENTIFIER(name) => frame.borrow().get(name).unwrap(),
            TreeCode::OPERATOR(op) => {
                let left = self.eval_expr(expr.borrow().children[0].clone(), frame.clone());
                let right = self.eval_expr(expr.borrow().children[1].clone(), frame.clone());
                if let (Value::INT(l), Value::INT(r)) = (left, right) {
                    match op.as_str() {
                        "+" => Value::INT(l + r),
                        "-" => Value::INT(l - r),
                        "*" => Value::INT(l * r),
                        "<" => Value::INT((l < r) as i64),
                        ">" => Value::INT((l > r) as i64),
                        _ => panic!("Unsupported operator"),
                    }
                } else { panic!("Invalid operands") }
            }
            TreeCode::FUNCTION_CALL(_) => {
                let expr_borrow = expr.borrow();
                let name_node = expr_borrow.children.get(0).unwrap();
                if let TreeCode::IDENTIFIER(name) = &name_node.borrow().token {
                    let mut args = vec![];
                    for a in expr.borrow().children.iter().skip(1) {
                        args.push(self.eval_expr(a.clone(), frame.clone()));
                    }
                    let func_node = self.functions.get(name)
                        .unwrap_or_else(|| panic!("Function `{}` not found", name));
                    self.call_function(func_node.clone(), args, Some(frame.clone()))
                } else { panic!("Expected function name") }
            }
            _ => panic!("Unsupported expression: {:?}", expr.borrow().token),
        }
    }
}

