fn main() {


    println!("{}", Solution::num_of_strings(
        vec![String::from("a"), String::from("abc"), String::from("ab"), String::from("d")],
        String::from("abc")
    ));
}
struct Solution {}

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        let mut count = 0;

        let mut j = 0;

        for i in patterns {
            j = 0;

            if i.len() > word.len() {
                break;
            }

            while j < word.len() - i.len() + 1 {
                if &word[j..j + i.len()] == &i {
                    count += 1;
                    break;
                }

                j += 1;
            }
        }

        count
    }
}
