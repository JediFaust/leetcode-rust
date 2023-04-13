fn main() {
    println!("{}", Solution::find_gcd(vec![2,5,6,9,10]));
    println!("{}", Solution::find_gcd(vec![7,5,6,8,3]));
}
struct Solution {}

impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];

        for i in 1..nums.len() {
            if nums[i] > max {
                max = nums[i];
            }

            if nums[i] < min {
                min = nums[i];
            }
        }

        gcd(min, max)
    }
}

fn gcd(first: i32, second: i32) -> i32 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}