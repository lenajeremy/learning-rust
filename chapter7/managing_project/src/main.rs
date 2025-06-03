mod data_types;
mod front_of_house;
mod garden;
mod stuff;

use data_types::Stack;
use garden::vegetable::Asparagus;

fn main() {
    garden::dance();
    println!("Hello, world!");
    let new_asparagus = Asparagus {};
    println!("{:?}", new_asparagus);

    let mut s = Stack::from_vec(vec![1, 2, 3, 4]);

    println!("{:#?}", s);
    println!("{}", s.peek().unwrap_or(-1));
    println!("{}", s.pop().unwrap_or(-1));
}
