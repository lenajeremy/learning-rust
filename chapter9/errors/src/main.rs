fn main() {
    println!("Hello, world!");
    read_username_from_file(String::from("src/main.rs")).unwrap();
    let content = read_username_from(String::from("src/main.rs")).unwrap();
    println!("{content}");
}

use std::{
    fs::File,
    io::{self, Read},
};

fn read_username_from_file(file: String) -> Result<String, io::Error> {
    let file = File::open(file);

    let mut username_file = match file {
        Ok(u) => u,
        Err(msg) => return Err(msg),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(msg) => Err(msg),
    }
}

fn read_username_from(file: String) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(file)?.read_to_string(&mut username)?;

    Ok(username)
}
