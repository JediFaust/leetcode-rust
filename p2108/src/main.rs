
fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for i in words {
            if i == i.chars().rev().collect::<String>() {
                return i;
            }
        }

        return String::from("");
    }
}

fn is_palindrome(s: String) -> bool {
    let mut i = 0;

    let l_s = s.as_bytes();

    while i < s.len() {
        if l_s[i] != l_s[s.len() - 1 - i] {
            return false;
        }

        i += 1;
    }

    return  true;
}