fn main() {
    println!("{}", Solution::remove_occurrences(String::from("eemckxmckx"), String::from("emckx")));
}

struct Solution {}

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut result = s.clone();

        let mut i = 0; 

        loop {
            if i + part.len() > result.len() {
                break;
            }

            if &result[i..i + part.len()] == part {
                if result.len() == part.len() {
                    result = String::from("");
                    break;
                }
                let part_one = String::from(&result[0..i]);
                let part_two = String::from(&result[i + part.len()..]);
                result = part_one;
                result.push_str(&part_two);

                if i > part.len() {
                    i -= part.len();
                } else {
                    i = 0;
                }
            } else {
                i += 1;
            }

            
        }
        result
    }
}