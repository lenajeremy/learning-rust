use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(&self, words: Vec<String>) -> Vec<Vec<String>> {
        fn hash(word: &str) -> [i32; 26] {
            let mut counter = [0; 26];

            for c in word.to_lowercase().chars() {
                let ascii_value = c as u8;
                counter[(ascii_value - 97) as usize] += 1;
            }

            counter
        }

        let mut dict: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        for word in words {
            let key = hash(&word);
            dict.entry(key).or_insert_with(Vec::new).push(word);
        }

        let res: Vec<Vec<String>> = dict.values().map(|x| x.clone()).collect();
        res
    }
}
