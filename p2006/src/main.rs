fn main() {
    println!("{}", Solution::count_k_difference(vec![1, 3], 3));
    println!("{}", Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2));
}

struct Solution {}

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;

        let mut i = 0;
        let mut j: usize;

        while i < nums.len() - 1 {
            j = i + 1;

            while j < nums.len() {

                if i32::abs(nums[i] - nums[j]) == k {
                    result += 1;
                }
                
                j += 1;
            }
            
            i += 1;
        }

        return result;
    }
}