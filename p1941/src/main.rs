fn main() {
    println!("{}", Solution::are_occurrences_equal("abc".to_string()));
}

use std::collections::HashMap;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut signs: HashMap<char, usize> = HashMap::new();

        let mut count = 0;

        for i in s.chars() {
            let sign = signs.entry(i).or_insert(0);
            
            *sign += 1;

            if *sign > count {
                count = *sign;
            }
        }

        for (_, v) in signs {
            if v != count {
                return false;
            }
        }

        true
    }
}

struct Solution {}