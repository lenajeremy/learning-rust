use dsa::graphs::{Node, bfs};
use std::{cell::RefCell, rc::Rc};

fn main() {
    let a = Rc::new(RefCell::new(Node::new("a")));
    let b = Rc::new(RefCell::new(Node::new("b")));
    let c = Rc::new(RefCell::new(Node::new("c")));
    let d = Rc::new(RefCell::new(Node::new("d")));
    let e = Rc::new(RefCell::new(Node::new("e")));
    let f = Rc::new(RefCell::new(Node::new("f")));
    let g = Rc::new(RefCell::new(Node::new("g")));

    a.borrow_mut().add_child(b.clone());
    a.borrow_mut().add_child(c.clone());

    b.borrow_mut().add_child(a.clone());
    b.borrow_mut().add_child(c.clone());

    c.borrow_mut().add_child(b.clone());
    c.borrow_mut().add_child(a.clone());
    c.borrow_mut().add_child(d.clone());

    d.borrow_mut().add_child(a.clone());
    d.borrow_mut().add_child(e.clone());

    e.borrow_mut().add_child(f.clone());

    println!("{}", bfs(a, f));
}
