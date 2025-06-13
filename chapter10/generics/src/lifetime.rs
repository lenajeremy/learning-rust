fn longest<'fll>(x: &'fll str, y: &'fll str) -> &'fll str {
    let s = "fjdad";
    let sample = if x.len() > y.len() { x } else { y };
    println!("{}", sample);
    s
}

pub fn test() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
