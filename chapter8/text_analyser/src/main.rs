use std::collections::HashMap;

use analyser::Analysis;

mod analyser;

fn main() {
    let content = analyser::read_file("../../chapter4/understanding_references/src/main.rs");
    let analysis = analyser::analyse(content);

    println!("Word Count: {}", analysis.word_count());

    //println!("{:#?}", analysis);
}
