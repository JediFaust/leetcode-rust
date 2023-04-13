struct Solution {}

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        let mut result = 0;
        let mut count = 0;

        for i in s.chars() {
            if i == '*' && count % 2 == 0 {
                result += 1;
            } else if i == '|' {
                count += 1;
            }
        }

        return result;
    }
}

fn main() {
    assert_eq!(Solution::count_asterisks(String::from("l|*e*et|c**o|*de|")), 2);

    println!("Hello, world!");
}
