use crate::stuff::Breakfast;

pub fn take_order() {}
fn serve_order() {}
fn take_payment() {}

pub struct Customer {
    pub name: String,
    pub total_amount_spent: f64,
}

fn get_total_calories() {
    let b = Breakfast {
        name: "bread and tea".to_string(),
    };
}
