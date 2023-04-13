
fn main() {
    println!("{}", Solution::find_final_value(vec![5,3,6,1,12], 3));
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut result = original;
        let mut set = HashSet::new();

        for i in nums {
            set.insert(i);

            if i == result {
                result *= 2;
            }
        }
        
        loop {
            let i = set.get(&result);
            
            match i {
                Some(v) => result = *v,
                None => break,
            }
        }

        result
    }
}