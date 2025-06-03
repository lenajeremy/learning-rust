use super::serving::Customer;

pub fn add_to_waitlist() {
    println!("Adding to waitlist");
}

fn seat_at_table() -> Customer {
    return super::serving::Customer {
        name: String::from("Jeremiah"),
        total_amount_spent: 50.0,
    };
}
