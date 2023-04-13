struct Solution {}

impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut first_max = 0;
        let mut second_max = 0;

        let mut first_min = i32::pow(10, 5);
        let mut second_min = i32::pow(10, 5);

        let mut i = 0;

        while i < nums.len() {
            if nums[i] > first_max {
                second_max = first_max;
                first_max = nums[i];
            } else if nums[i] > second_max {
                second_max = nums[i];
            }

            if nums[i] < first_min {
                second_min = first_min;
                first_min = nums[i];
            } else if nums[i] < second_min {
                second_min = nums[i];
            }

            i += 1;
        }

        println!("f_max: {}", first_max);
        println!("s_max: {}", second_max);
        println!("f_min: {}", first_min);
        println!("s_min: {}", second_min);

        result = (first_max * second_max) - (first_min * second_min);

        return result;
    }
}

fn main() {
    println!("{}", Solution::max_product_difference(vec![5,6,2,7,4]));
}
