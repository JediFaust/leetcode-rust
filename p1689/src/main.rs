struct Solution {}

impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut max: i32 = 0;
        
        for i in n.chars() {
            if i.to_digit(10).unwrap() as i32 > max {
                max = i.to_digit(10).unwrap() as i32;
            }
        }

        return max;
    }
}

fn main() {
    println!("{}", Solution::min_partitions(String::from("82734")));
}
