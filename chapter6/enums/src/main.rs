fn main() {
    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let seven = plus_one(six);

    println!(
        "Five: {:?}, Six: {:?}, Seven: {:?}, None: {:?}",
        five, six, seven, none
    );

    let x = none.unwrap();
    println!("{x}");
}

enum Love {
    Agape,
    Platonic(String),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
