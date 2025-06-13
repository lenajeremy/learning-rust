fn main() {
    let user = User {
        username: "Jeremiah".to_string(),
        email: "jeremiahlena13@gmail.com".to_string(),
        active: true,
        sign_in_count: 50,
    };

    println!("Email: {}", user.email);
    println!("Username: {}", user.username);
    println!("Active: {}", user.active);
    println!("Sign In Count: {}", user.sign_in_count);
    println!("{:#?}", user);

    user.greet();

    let u = User::from(String::from("John Doe"), String::from("johndoe@email.com"));

    println!("{:#?}", u);
    u.greet();
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    fn greet(self: &Self) {
        println!("{} is saying hi", self.username);
    }

    fn from(name: String, email: String) -> Self {
        Self {
            username: name,
            email: email,
            active: false,
            sign_in_count: 9,
        }
    }
}
