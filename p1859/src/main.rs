
fn main() {
    println!("{}", Solution::sort_sentence("is2 sentence4 This1 a3".to_string()));
}

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words: HashMap<String, &str> = HashMap::new();

        for i in s.split(" ") {
            words.insert(i[i.len()-1..i.len()].to_string(), i);
        }

        let mut result = String::new();

        for i in 1..words.len() as i32 + 1 {
            let item = words.get(&i.to_string()).unwrap();
            result.push_str(&item[0..item.len() - 1]);

            if i != words.len() as i32 {
                result.push(' ');
            }
        }

        result
    }
}