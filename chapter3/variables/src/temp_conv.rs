use std::io;

enum Unit {
    F,
    C,
}

fn main() {
    println!("Welcome to temperature converter");

    loop {
        let mut temperature = String::new();
        let mut unit: Unit;
        print!("Enter temperature: ");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Invalid temperature");
    }
}
