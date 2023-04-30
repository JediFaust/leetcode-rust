struct Solution {}

impl Solution {
    pub fn left_rigth_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        for i in 0..nums.len() {
            result.push(0);
            for j in 0..nums.len() {
                if i > j { result[i] += nums[j]; }
                else if i < j { result[i] -= nums[j]; }
            }
            if result[i] < 0 { result[i] *= -1; }
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::left_rigth_difference(
        Vec::from([10,4,8,3])
    ));
}