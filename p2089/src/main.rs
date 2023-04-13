fn main() {
    println!("{:?}", Solution::target_indices(vec![1,2,5,2,3], 2));
}

struct Solution {}

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();

        let mut nums = nums;

        nums.sort();

        for i in 0..nums.len() {
            if nums[i] == target {
                result.push(i as i32);
            }
        }
        result
    }
}