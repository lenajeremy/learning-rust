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

    //println!("{sl}");

    println!("{s}");
}
