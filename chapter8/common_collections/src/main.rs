mod group_anagrams;

use std::collections::HashMap;

fn main() {
    println!("Chapter 8");
    let s = group_anagrams::Solution {};
    let groups = s.group_anagrams(vec![
        String::from("tea"),
        String::from("ate"),
        String::from("eat"),
        String::from("efe"),
        String::from("fee"),
        String::from("low"),
        String::from("owl"),
    ]);

    println!("{:?}", groups);
    hashmaps();
    strings();
}

fn hashmaps() {
    let mut cache = HashMap::new();

    cache.insert(3, String::from("Three"));
    cache.insert(4, "four".to_string());

    println!("{cache:?}");

    for (key, value) in &cache {
        println!("Key: {key}, Value: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn strings() {
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

    println!("{}", sl.to_uppercase());

    s.push_str(&String::from("Glorify Him!"));

    println!("{s}");
}
