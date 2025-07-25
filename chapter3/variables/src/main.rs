use std::io;

//enum Unit {
//    F,
//    C,
//}

fn main() {
    fib_interactive();
    temp_converter();
}

fn fib_interactive() {
    println!("Welcome to interactive fibonacci");

    fn fib(n: usize) -> usize {
        if n <= 1 {
            return n;
        }

        let res = fib(n - 1) + fib(n - 2);
        return res;
    }

    loop {
        let mut n = String::new();
        println!("\n\nEnter your number:");
        io::stdin().read_line(&mut n).expect("Invalid number");

        let n: usize = match n.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        println!("The {n}th fibonacci number is {}", fib(n));
    }
}

fn temp_converter() {
    println!("Welcome to temperature converter\n\n");

    loop {
        let mut temperature = String::new();
        let mut unit = String::new();

        println!("Enter temperature: ");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Invalid temperature");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };

        println!("");

        println!("Enter temperature unit: ");
        io::stdin().read_line(&mut unit).expect("Invalid unit");
        let unit = unit.trim();

        //let unit: Unit = match unit.trim() {
        //    "F" => Unit::F,
        //    "C" => Unit::C,
        //};
        //

        let converted: f64;
        if unit == "F" {
            converted = (temperature - 32.0) * (5.0 / 9.0);
            println!("{temperature}F equals {converted}C")
        } else if unit == "C" {
            converted = temperature * (9.0 / 5.0) + 32.0;
            println!("{temperature}C equals {converted}F");
        } else {
            println!("Invalid unit");
        }
    }
}
