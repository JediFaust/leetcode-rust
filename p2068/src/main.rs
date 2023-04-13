fn main() {
    println!("Hello, world!");
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut f_count = HashMap::new();
        let mut s_count = HashMap::new();

        for i in word1.chars() {
            let ch = f_count.entry(i).or_insert(0);
            *ch += 1;
        }

        for i in word2.chars() {
            let ch = s_count.entry(i).or_insert(0);
            *ch += 1;
        }

        for (k, v) in &f_count {
            let i = s_count.get(&k).or(Some(&0)).unwrap();

            if i32::abs(i - v) > 3 {
                return false;
            } 
        }

        for (k, v) in s_count {
            let i = f_count.get(&k).or(Some(&0)).unwrap();

            if i32::abs(i - &v) > 3 {
                return false;
            }
        }

        true
    }
}