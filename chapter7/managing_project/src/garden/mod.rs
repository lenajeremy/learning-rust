pub mod vegetable;

pub fn dance() {
    println!("I am dancing");
    let asp = vegetable::Asparagus {};
    println!("{:?}", asp);
}
