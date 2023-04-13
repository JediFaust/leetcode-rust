use std::collections::HashMap;

fn main() {
    println!("{}", Solution::decode_message(
        "the quick brown fox jumps over the lazy dog".to_string(),
        "vkbs bs t suepuv".to_string()
    ));
}

impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut id = 0;

        let mut result = String::new();

        let mut decoder: HashMap<char, char> = HashMap::new();

        decoder.insert(' ', ' ');

        for i in key.chars() {
            if !decoder.contains_key(&i) {
                decoder.insert(i, alpha[id]);
                id += 1;
            }
        }

        for i in message.chars() {
            result.push(decoder[&i]);
        }

        result
    }
}

struct Solution {}