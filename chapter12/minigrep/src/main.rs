use minigrep::{Config, run};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let config = Config::build(&args);

    if let Err(msg) = run(config) {
        println!("Application Error: {msg}");
        std::process::exit(1);
    };
}
fn test_lifetimes<'a>(v: &'a [String], s: &'a [String]) -> &'a [String] {
    let mut new_slice = vec![];

    for word in v {
        new_slice.push(word);
    }

    for word in s {
        new_slice.push(word);
    }
    if v[0] > s[0] { v } else { s }
}
