fn main() {
    println!("Hello, world!");
}

struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut unique = HashSet::new();

        for i in emails {
            let mut trigger = true;
            let mut email = String::new();

            for j in i
        }

        unique.len() as i32
    }
}
