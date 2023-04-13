fn main() {
    println!("{}", Solution::count_words(vec![
        "leetcode".to_string(),"is".to_string(),"amazing".to_string(),"as".to_string(),"is".to_string()
        ], vec!["amazing".to_string(),"leetcode".to_string(),"is".to_string()]));
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut result = 0;

        let mut f_count = HashMap::new();
        let mut s_count = HashMap::new();

        for i in words1 {
            let item = f_count.entry(i).or_insert(0);

            *item += 1;
        }

        for i in words2 {
            let item = s_count.entry(i).or_insert(0);

            *item += 1;
        }

        for k in f_count.keys() {
            if let Some(v) = f_count.get(k) {
                if let Some(val) = s_count.get(k) {
                    if v == &1 && val == &1 {
                        result += 1;
                    }
                }
            }
        }

        result
    }
}