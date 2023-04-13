fn main() {
    println!("{}", Solution::count_largest_group(13));
    println!("{}", Solution::count_largest_group(1));
    println!("{}", Solution::count_largest_group(20));
    println!("{}", Solution::count_largest_group(24));
    println!("{}", Solution::count_largest_group(33));
}

struct Solution {}

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut result = 0;

        let mut num = n;

        loop {
            if num > 9 {
                num -= 9;
            } else {
                result = num;
                break;
            }
        }
        result
    }
}