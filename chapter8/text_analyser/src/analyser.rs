use std::collections::HashMap;

#[derive(Debug)]
pub struct Analysis {
    word_count: usize,
    frequency_map: HashMap<String, usize>,
    average_word_length: usize,
}

impl Analysis {
    fn get_top_k_words(k: usize) -> Vec<String> {
        println!("{}", k);
        vec![]
    }
}

pub fn read_file(file: &str) -> String {
    let mut content = std::fs::read_to_string(file).unwrap().to_lowercase();
    let mut content = content
        .chars()
        .filter(|x| x.is_alphanumeric() || x.is_whitespace())
        .collect();
    content
}

pub fn analyse(content: String) -> Analysis {
    Analysis {
        word_count: get_word_count(&content),
        frequency_map: get_frequency_map(&content),
        average_word_length: average_word_length(&content),
    }
}

fn get_word_count(content: &str) -> usize {
    content.split_whitespace().count()
}

fn get_frequency_map(content: &String) -> HashMap<String, usize> {
    let mut hashmap: HashMap<String, usize> = HashMap::new();

    for word in content.split_whitespace() {
        hashmap
            .entry(String::from(word))
            .and_modify(|prev| *prev += 1)
            .or_insert(1);
    }
    hashmap
}

fn average_word_length(content: &String) -> usize {
    let mut total_word_length = 0;

    for word in content.split_whitespace() {
        total_word_length += word.chars().count();
    }

    let total_words = get_word_count(content);

    total_word_length / total_words
}
