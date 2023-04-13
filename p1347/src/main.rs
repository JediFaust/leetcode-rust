use std::collections::HashMap;

fn main() {
    println!("{}", Solution::min_steps("bab".to_string(), "aba".to_string()));
    println!("{}", Solution::min_steps("leetcode".to_string(), "practice".to_string()));
    println!("{}", Solution::min_steps("anagram".to_string(), "mangaar".to_string()));
}

struct Solution {}

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut result = 0;
        
        let mut s_count = HashMap::new();
        let mut t_count = HashMap::new();

        for i in s.chars() {
            let current = s_count.entry(i).or_insert(0);

            *current += 1;
        }

        for i in t.chars() {
            let current = t_count.entry(i).or_insert(0);

            *current += 1;
        }

        for (k, v) in s_count {
            match t_count.get(&k) {
                Some(val) => {
                    if &v > val {
                        result += v - val;
                    }
                    println!("V: {}, Value: {}", v, val)
                },
                None => result += v,
            }
        }

        result
    }
}