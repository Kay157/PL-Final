use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::mtree::MTree;
use crate::tokens::TreeCode;

//scope stack
struct ScopeStack {
    stack: Vec<HashMap<String, ()>>,
}

impl ScopeStack {
    fn new() -> Self {
        ScopeStack {
            stack: vec![HashMap::new()],
        }
    }

    fn push(&mut self) { self.stack.push(HashMap::new()); }
    fn pop(&mut self) { self.stack.pop(); }

    fn declare(&mut self, name: &str) {
        self.stack.last_mut().unwrap().insert(name.to_string(), ());
    }

    fn is_declared(&self, name: &str) -> bool {
        for scope in self.stack.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }
}

// analysis
pub fn analyze(root: Rc<RefCell<MTree>>) {
    let mut scopes = ScopeStack::new();
    analyze_node(root, &mut scopes, None);
}

fn is_function_name(node: &Rc<RefCell<MTree>>, parent: &Rc<RefCell<MTree>>) -> bool {
    if let TreeCode::FUNCTION = parent.borrow().token {
        if let Some(first) = parent.borrow().children.get(0) {
            return Rc::ptr_eq(first, node);
        }
    }
    false
}

fn analyze_node(
    node: Rc<RefCell<MTree>>,
    scopes: &mut ScopeStack,
    parent: Option<Rc<RefCell<MTree>>>
) {
    let n = node.borrow();

    match &n.token {

        // program predeclare func names
        TreeCode::PROGRAM => {

            for c in &n.children {
                let func = c.borrow();
                if let TreeCode::FUNCTION = func.token {
                    if let Some(name_node) = func.children.get(0) {
                        if let TreeCode::IDENTIFIER(name) = &name_node.borrow().token {
                            scopes.declare(name); // <-- FIX
                        }
                    }
                }
            }

            for c in &n.children {
                analyze_node(c.clone(), scopes, Some(node.clone()));
            }
        }

        // func children = [name, params, block]
        TreeCode::FUNCTION => {
            scopes.push();

            //params
            if let Some(param_list) = n.children.get(1) {
                let pl = param_list.borrow();
                for p in &pl.children {
                    if let TreeCode::PARAMETER = &p.borrow().token {
                        if let Some(id) = p.borrow().children.get(0) {
                            if let TreeCode::IDENTIFIER(name) = &id.borrow().token {
                                scopes.declare(name);
                            }
                        }
                    }
                }
            }

            //block
            if let Some(block) = n.children.get(2) {
                analyze_node(block.clone(), scopes, Some(node.clone()));
            }

            scopes.pop();
        }

        TreeCode::BLOCK => {
            scopes.push();
            for c in &n.children {
                analyze_node(c.clone(), scopes, Some(node.clone()));
            }
            scopes.pop();
        }

        TreeCode::LET => {
            if let Some(id) = n.children.get(0) {
                if let TreeCode::IDENTIFIER(name) = &id.borrow().token {
                    scopes.declare(name);
                }
            }
        }

        // identifier usage except when func name
        TreeCode::IDENTIFIER(name) => {
            if let Some(parent_node) = parent {
                if is_function_name(&node, &parent_node) {
                    // Function names are not variable uses
                } else if !scopes.is_declared(name) {
                    eprintln!("SEMANTIC ERROR: variable `{}` used before declaration", name);
                }
            }
        }

        //assign
        TreeCode::ASSIGN => {
            if let Some(id) = n.children.get(0) {
                if let TreeCode::IDENTIFIER(name) = &id.borrow().token {
                    if !scopes.is_declared(name) {
                        eprintln!(
                            "SEMANTIC ERROR: assigning to undeclared variable `{}`",
                            name
                        );
                    }
                }
            }

            if let Some(expr) = n.children.get(1) {
                analyze_node(expr.clone(), scopes, Some(node.clone()));
            }
        }

        // recursively analyze children
        TreeCode::PRINT |
        TreeCode::IF |
        TreeCode::WHILE |
        TreeCode::RETURN |
        TreeCode::OPERATOR(_) |
        TreeCode::FUNCTION_CALL(_) |
        TreeCode::INT_LITERAL(_) |
        TreeCode::BOOL_LITERAL(_) |
        TreeCode::EOF |
        TreeCode::PARAM_LIST |
        TreeCode::PARAMETER |
        TreeCode::STATEMENT |
        TreeCode::EXPRESSION =>
            {
                for c in &n.children {
                    analyze_node(c.clone(), scopes, Some(node.clone()));
                }
            }
    }
}