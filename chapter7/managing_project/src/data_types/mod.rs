#[derive(Debug)]
pub struct Stack {
    items: Vec<i32>,
}

impl Stack {
    pub fn pop(&mut self) -> Option<i32> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<i32> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items[0])
        }
    }

    pub fn push(&mut self, value: i32) -> i32 {
        self.items.push(value);
        value
    }

    pub fn from_vec(vec: Vec<i32>) -> Stack {
        Stack { items: vec }
    }
}
