fn main() {
    println!("{}", Solution::divide_array(vec![0,0,1,1,2,2]));
    println!("{}", Solution::divide_array(vec![1,0,1,1,2,2]));
}

use std::collections::HashMap;

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut count = HashMap::new();

        for i in nums {
            let val = count.entry(i).or_insert(0);
            *val += 1;
        }

        for (_, v) in count {
            if v % 2 != 0 {
                return false;
            }
        }

        true
    }
}

struct Solution {}