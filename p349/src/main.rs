
fn main() {
    println!("{:?}", Solution::intersection(vec![1,2,2,1], vec![2,2]));
    println!("{:?}", Solution::intersection(vec![4,9,5], vec![9,4,9,8,4]));
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        
        let mut set_1 = HashSet::new();
        let mut set_2 = HashSet::new();

        for i in nums1 {
            set_1.insert(i);
        }

        for i in nums2 {
            set_2.insert(i);
        }

        for i in set_1 {
            if set_2.contains(&i) {
                result.push(i);
            }
        }

        result
    }
}  