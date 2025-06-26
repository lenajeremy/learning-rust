use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let b = Box::new("hello");
    println!("{b}");
    //init();
    another_test();
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn init() {
    let n = Rc::new(Cons(49, Rc::new(Cons(40, Rc::new(Nil)))));
    let _b = Cons(50, Rc::clone(&n));
    let c = Cons(80, Rc::clone(&n));

    println!("{}", Rc::strong_count(&n));
    match &c {
        Nil => return (),
        Cons(i, cl) => {
            println!("i = {}, c = {:?}", i, cl);
        }
    }

    println!("{:?}", c);

    println!("{}", Rc::strong_count(&n));
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
//
pub trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    max: usize,
    value: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger: messenger,
            max: max,
            value: 0,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let consumption = self.value as f64 / self.max as f64;
        if consumption > 1.0 {
            self.messenger.send("you have exceeded your set limit");
        } else if consumption > 0.9 {
            self.messenger
                .send("you have exceeded over 90% your set limit");
        } else if consumption > 0.75 {
            self.messenger
                .send("you have exceeded over 75% your set limit");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct MockMessenger {
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            {
                let mut t = self.messages.borrow_mut();
                t.push("Jeremiah is a fine boyyyyyy".to_string());
                t.push(msg.to_string());
            }
            self.messages.borrow_mut().pop().unwrap();
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_usage_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert!(mock_messenger.messages.borrow().len() == 1);
    }
}

fn another_test() {
    struct MyLargeStruct;

    let s = MyLargeStruct {};
    let s_ref = Rc::new(&s);

    println!("{}", Rc::strong_count(&s_ref));
    let t = s_ref.clone();
    println!("{}", Rc::strong_count(&s_ref));
}
