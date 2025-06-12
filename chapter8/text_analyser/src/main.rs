mod analyser;

fn main() {
    let content = analyser::read_file("../../chapter4/understanding_references/src/main.rs");
    let analysis = analyser::analyse(content);

    println!("{:#?}", analysis);
}
