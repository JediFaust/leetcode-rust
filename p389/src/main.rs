fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let alpha: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect();
        let mut lib = [0; 26];

        for i in s.chars() {
            for j in 0..alpha.len() {
                if i == alpha[j] {
                    lib[j] += 1;
                    break;
                }
            }
        }

        for i in t.chars() {
            for j in 0..alpha.len() {
                if i == alpha[j] {
                    if lib[j] == 0 {
                        return alpha[j];
                    }

                    lib[j] -= 1;
                    break;
                }
            }
        }
        ' '
    }
}