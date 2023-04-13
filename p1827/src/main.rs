fn main() {
    println!("{}", Solution::min_operations(vec![1,1,1]));
    println!("{}", Solution::min_operations(vec![1,5,2,4,1]));
}

struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut last = nums[0];

        for i in 1..nums.len() {
            if last >= nums[i] {
                result += last - nums[i] + 1;
                last += 1;
            } else {
                last = nums[i];
            }
        }

        result
    }
}