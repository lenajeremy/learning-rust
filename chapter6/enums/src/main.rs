use std::io::{self, Write};

fn main() {
    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let seven = plus_one(six);

    println!("Five: {:?}, Six: {:?}, Seven: {:?}", five, six, seven);

    test();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum Reason {
    CelebratingBirthday(i32, i32, i32),
    Sick(String),
    OnLeave,
}

fn test() {
    let reason = Reason::Sick(String::from("Malaria"));
    let verdict: String = match &reason {
        Reason::CelebratingBirthday(day, month, year) => {
            println!("Ensure to send happy birthday mail to him/her");
            format!("Happy birthday to our dear employee {day}/{month}/{year}")
        }
        Reason::OnLeave => {
            format!("No verdict")
        }
        Reason::Sick(sickness) => {
            format!("Get well soon, i hope your {sickness} goes away soon")
        }
        _ => format!("Invalid reason... you should be sacked"),
    };

    println!("{:?}", reason);
    println!("{verdict}");
}
