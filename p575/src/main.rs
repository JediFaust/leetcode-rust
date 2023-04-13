
fn main() {
    println!("{:?}", Solution::distribute_candies(vec![1,1,2,2,3,3]));
    println!("{:?}", Solution::distribute_candies(vec![1,1,2,3]));
    println!("{:?}", Solution::distribute_candies(vec![6,6,6,6]));
    println!("{:?}", Solution::distribute_candies(vec![6,6,6,6,6]));
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut u_candies = HashSet::new();

        for i in &candy_type {
            u_candies.insert(i);
        }

        if u_candies.len() < candy_type.len() / 2 {
            return u_candies.len() as i32;
        } else {
            return (candy_type.len() / 2) as i32;
        }
    }
}