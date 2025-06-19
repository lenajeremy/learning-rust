use std::{
    fmt::Debug,
    ops::{Add, Sub},
    thread,
};

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_count = 0;
        let mut blue_count = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => red_count += 1,
                ShirtColor::Blue => blue_count += 1,
            };
        }

        if red_count > blue_count {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn _manage_inventory() {
    let inventory = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let u_pref1 = Some(ShirtColor::Blue);
    let giveaway1 = inventory.giveaway(u_pref1);
    println!(
        "User with preference {:?} got shirt {:?}",
        u_pref1, giveaway1
    );

    let u_pref2 = None;
    let giveaway2 = inventory.giveaway(u_pref2);
    println!(
        "User with preference {:?} got shirt {:?}",
        u_pref2, giveaway2
    );
}

fn generate_workout() {
    #[derive(Debug)]
    struct Sample {
        value: i32,
    }

    impl Add for Sample {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Sample {
                value: self.value + rhs.value,
            }
        }
    }

    impl Sub for Sample {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Sample {
                value: self.value - rhs.value,
            }
        }
    }

    let joined_sample = sample_closure(Sample { value: 5 }, Sample { value: 10 });
    println!("{:?}", &joined_sample);
    println!("{:?}", joined_sample - Sample { value: 15 });
}

fn main() {
    generate_workout();
    sort_list();
    closures()
}

fn closures() {
    let mut list = vec![""];

    let mut mutably_borrow = || list.push("hello");

    mutably_borrow();
    mutably_borrow();
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}

fn sample_closure<T>(x: T, y: T) -> T::Output
where
    T: Add + Debug,
{
    println!("X: {:?}, Y: {:?}", x, y);
    println!("Hello world");
    x + y
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sort_list() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
