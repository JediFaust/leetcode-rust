fn main() {
    println!("{}", Solution::count_good_substrings("xyzzaz".to_string()));
}

struct Solution {}

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let mut result = 0;
        let s_vec: Vec<char> = s.chars().collect();

        if s.len() >= 3 {
            let mut i = 0;
            while i < s.len() - 2 {

                if s_vec[i] != s_vec[i + 1] &&  s_vec[i + 1]!= s_vec[i + 2] && s_vec[i] != s_vec[i + 2] {
                    result += 1;
                }

                i += 1;
            }
        }
        result
    }
}