struct Solution {}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut i = 0;

        while i < nums.len() * 2 {
            result.push(nums[i % nums.len()]);

            i += 1
        }

        return result;
    }
}

fn main() {
    let some = Solution::get_concatenation(vec![1, 2, 1]);

    println!("{:?}", some);
}