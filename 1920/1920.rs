use std::convert::TryInto;
use std::convert::TryFrom;

struct Solution {}

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut i: usize = 0;

        while i < nums.len() {
            result.push(nums[try_into(nums[i]).unwrap()]);

            i += 1;
        }

        return result;
    }
}

fn main() {
    let mut nums_test = vec![0, 2, 1, 5, 3, 4];

    println!("{:?}", Solution::build_array(nums_test));
}