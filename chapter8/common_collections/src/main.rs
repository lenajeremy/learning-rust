use std::collections::{HashMap, hash_map::Entry};

fn main() {
    println!("Chapter 8");
    // storing vectors
    //
    let mut v = vec![5; 10];
    println!("{v:#?}");

    v.push(5);

    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שלום");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");

    println!("{hello}");

    let mut s = "hello world".to_string();
    s.push_str(". The LORD is King!");
    let sl = &s[0..=7];

    println!("{}", s.to_uppercase());
    println!("{s}");

    println!("{sl}");

    s.push_str(&String::from(" Glorify Him!"));

    println!("{s}");

    hashmaps();
}

fn hashmaps() {
    let mut cache = HashMap::new();

    cache.insert(3, String::from("Three"));
    cache.insert(4, "four".to_string());

    println!("{cache:?}");

    for (key, value) in &cache {
        println!("Key: {key}, Value: {value}");
    }

    let v = vec![4; 100];
    let sum: usize = v.iter().sum();
    println!("{sum}");
}
