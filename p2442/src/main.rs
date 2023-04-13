use std::{collections::HashMap};

fn main() {
    println!("Hello, world!");

    assert_eq!(Solution::count_distinct_integers(vec![1,13,10,12,31]), 6);
    assert_eq!(Solution::count_distinct_integers(vec![2,2,2]), 1);
}

struct Solution {}

impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut distinct_numbers: HashMap<i32, bool>= HashMap::new();

        for i in nums {
            distinct_numbers.insert(i, true);
            distinct_numbers.insert(reverse(i), true);
        }

        return distinct_numbers.len() as i32;
    }
}

fn reverse(s: i32) -> i32 {
    let mut result = String::from("");

    for i in s.to_string().chars().rev() {
        result.push(i);
    }

    return result.parse::<i32>().unwrap();
}