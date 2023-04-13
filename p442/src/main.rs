struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut count = HashSet::new();

        for i in nums {
            if count.contains(&i) {
                result.push(i);
            } else {
                count.insert(i);
            }
        }

        result
    }
}

fn main() {
    println!("{:?}", Solution::find_duplicates(vec![4,3,2,7,8,2,3,1]));
    println!("{:?}", Solution::find_duplicates(vec![1,1,2]));
    println!("{:?}", Solution::find_duplicates(vec![1]));
}
