use std::rc::Rc;
use std::cell::RefCell;
use crate::tokens::TreeCode;

pub struct MTree {
    pub token : TreeCode,
    pub children : Vec<Rc<RefCell<MTree>>>
}

impl MTree {

    pub fn new(token : TreeCode) -> Rc<RefCell<MTree>> {
        Rc::new(RefCell::new(MTree {
            token,
            children : vec![]
        }))
    }

    pub fn _push(&mut self, tree : Rc<RefCell<MTree>>) {
        self.children.push(tree);
    }

    pub fn node_string(&self) -> String {
        format!("{:?}", self.token)
    }
    
    fn print_recursively(&self, level : usize) {
        let shift = 2*level;
        print!("{:1$}", "", shift);
        println!("{}", self.node_string());
        for child_rc in &self.children {
            child_rc.borrow().print_recursively(level+1);
        }
    }

    pub fn print(&self) {
        self.print_recursively(0);
    }
}
