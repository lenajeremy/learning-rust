use crate::queue;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::rc::Rc;

pub struct Node<T> {
    val: T,
    children: Vec<NodeType<T>>,
}

impl<T> Debug for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node(val={})", self.val)
    }
}

type NodeType<T> = Rc<RefCell<Node<T>>>;

impl<T> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node {
            val: val,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: NodeType<T>) {
        self.children.push(child);
    }
}

pub fn bfs(start: NodeType<&'static str>, end: NodeType<&'static str>) -> usize {
    let mut queue = queue::Queue::new();
    let mut s: HashSet<&str> = HashSet::new();

    s.insert(start.borrow().val);
    queue.enqueue((start, 0));

    while !queue.is_empty() {
        let (last, distance) = queue.dequeue().unwrap();
        if last.borrow().val == end.borrow().val {
            return distance;
        }

        for child in &last.borrow().children {
            if !s.contains(child.borrow().val) {
                queue.enqueue((child.clone(), distance + 1));
                s.insert(child.borrow().val);
            }
        }
    }
    0
}

pub fn dfs<T>(start: NodeType<T>, end: NodeType<T>) -> usize {
    fn traverse<T>(node: NodeType<T>) {}

    traverse(start);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bfs() {
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

        assert_eq!(bfs(a.clone(), f), 4);
        assert_eq!(bfs(a.clone(), a.clone()), 0);
        assert_eq!(bfs(a.clone(), c), 1);
        assert_eq!(bfs(a, d), 2);
    }
}
