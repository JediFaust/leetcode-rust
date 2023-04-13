struct Solution {}

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut result = String::new();
        let mut flag = true;

        for i in word.chars() {
            if i == ch && flag {
                result.push(i);
                result = result.chars().rev().collect();
                flag = false;
            } else {
                result.push(i);
            }
        }

        result
    }
}

fn main() {
    println!("{}", Solution::reverse_prefix(String::from("abcdefg"), 'd'));
}
