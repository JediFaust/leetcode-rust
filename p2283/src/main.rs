fn main() {
    println!("{}", Solution::digit_count("1210".to_string()));
    println!("{}", Solution::digit_count("030".to_string()));
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn digit_count(num: String) -> bool {
        let mut result = true;

        let mut count = HashMap::new();

        for i in num.chars() {
            let unit = count.entry(i.to_digit(10).unwrap()).or_insert(0);

            *unit += 1;
        }

        let mut j = 0;

        for i in num.chars() {
            let occuration = count.entry(j).or_insert(0);
            let sign = i.to_digit(10).unwrap();
            
            if sign != *occuration {
                result = false;
                break;
            }

            j += 1;
        }

        result
    }
}
