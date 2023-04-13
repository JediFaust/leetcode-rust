
fn main() {
    println!("{:?}", Solution::find_words(vec![
        "Hello".to_string(), "Alaska".to_string(), "Dad".to_string(), "Peace".to_string()
        ]));
}

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();

        let first = "qwertyuiop".to_string();
        let second = "asdfghjkl".to_string();
        let third = "zxcvbnm".to_string();

        let mut rows = HashMap::new();

        for i in first.chars() {
            rows.insert(i, 1);
        }

        for i in second.chars() {
            rows.insert(i, 2);
        }
        
        for i in third.chars() {
            rows.insert(i, 3);
        }

        for i in words {
            let mut trigger = true;
            let mut index = 0;
            if let Some(val) = rows.get(&i[0..1].to_lowercase().chars().next().unwrap_or(' ')) {
                index = *val;
            }

            for j in i.to_lowercase().chars() {
                if let Some(v) = rows.get(&j) {
                    if v != &index {
                        trigger = false;
                        break;
                    }
                }
            }

            if trigger {
                result.push(i);
            }
        }
        result
    }
}