fn main() {
    let mut args: Vec<_> = std::env::args().collect();
    let search_string = &args[1];
    let file_path = &args[2];
    println!("{args:?}");
    dbg!(search_string);
    dbg!(file_path);

    args.push("jeremiah".to_string());
}
