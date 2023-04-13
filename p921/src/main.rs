fn main() {
    println!("{}", Solution::min_add_to_make_valid(
        String::from("())")
    ));

    println!("{}", Solution::min_add_to_make_valid(
        String::from("(())")
    ));

    println!("{}", Solution::min_add_to_make_valid(
        String::from("))()))")
    ));
}

struct Solution {}

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open = 0;
        let mut close = 0;

        for i in s.chars() {
            if i == '(' {
                open += 1;
            } else {
                if open > 0 {
                    open -= 1;
                } else {
                    close += 1;
                }
            }
        }

        open + close
    }
}
