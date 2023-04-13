
fn main() {
    println!("{}", Solution::repeated_character("abccbaacz".to_string()));
    println!("{}", Solution::repeated_character("absdd".to_string()));
    println!("{}", Solution::repeated_character("hjhk".to_string()));
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut chars = HashMap::new();
        let mut result = ' ';
        for i in s.chars() {
            match chars.get(&i) {
                Some(_) => {
                    result = i;
                    break;
                },
                None => { chars.insert(i, true); },
            }
        }

        result
    }
}