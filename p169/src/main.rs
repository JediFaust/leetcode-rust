fn main() {
    println!("{}", Solution::majority_element(vec![2,2,1,1,1,2,2]));
    println!("{}", Solution::majority_element(vec![3,2,3]));
}

struct Solution {}

use std::{collections::HashMap};
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        for i in nums {
            let item = count.entry(i).or_insert(0);

            *item += 1;
        }

        let mut max = 0;
        let mut result = 0;

        for (k, v) in count {
            if v > max {
                max = v;
                result = k;
            }
        }
        
        result
    }
}