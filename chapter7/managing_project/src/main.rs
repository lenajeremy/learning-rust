mod front_of_house;
mod garden;
mod stuff;

use garden::vegetable::Asparagus;

fn main() {
    garden::dance();
    println!("Hello, world!");
    let new_asparagus = Asparagus {};
    println!("{:?}", new_asparagus);
}
