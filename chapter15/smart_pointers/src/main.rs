use std::rc::Rc;
use std::{
    fmt::{self, Display},
    ops::Deref,
};

fn main() {
    let b = Box::new("hello");
    println!("{b}");
    init();
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn init() {
    let n = Rc::new(Cons(49, Rc::new(Cons(40, Rc::new(Nil)))));
    let b = Cons(50, Rc::clone(&n));
    let c = Cons(80, Rc::clone(&n));

    println!("{}", Rc::strong_count(&n));
    //std::mem::drop(c);
    drop(c);
    println!("{}", Rc::strong_count(&n));

    //println!("{:?}", c);
}

//struct MyBox<T>(T);
//impl<T> MyBox<T> {
//    fn new(v: T) -> Self {
//        MyBox(v)
//    }
//}
//
//impl<T> Deref for MyBox<T> {
//    type Target = T;
//
//    fn deref(&self) -> &Self::Target {
//        println!("dereferencing mybox");
//        &self.0
//    }
//}
//
//impl<T> Drop for MyBox<T> {
//    fn drop(&mut self) {
//        println!("assuming this thing is being dropped");
//    }
//}
//
//impl<T> Display for MyBox<T>
//where
//    T: Display,
//{
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "{}", self.0)
//    }
//}
