fn main() {
    println!("{}", Solution::minimum_sum(4009));
    println!("{}", Solution::minimum_sum(2932));

}

struct Solution {}

impl Solution {
    pub fn minimum_sum(num: i32) -> i32 {
        let s = num.to_string();

        let first: i32 = s[0..1].parse().unwrap();
        let second = s[1..2].parse().unwrap();
        let third = s[2..3].parse().unwrap();
        let fourth = s[3..4].parse().unwrap();

        let mut v = vec![first, second, third, fourth];

        v.sort();

        ((v[0] * 10) + v[2] ) + ((v[1] * 10) + v[3])

        
        
    }
}