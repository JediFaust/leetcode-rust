fn main() {
    println!("{}", Solution::count_prefixes(vec![
        "a".to_string(),"b".to_string(),"c".to_string(),"ab".to_string(),"bc".to_string(),"abc".to_string()
        ], "abc".to_string()));
    println!("{}", Solution::count_prefixes(vec!["a".to_string(), "a".to_string()], "aa".to_string()));
}

struct Solution {}

impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let mut result = 0;

        for i in words {
            
            if i.len() <= s.len() {
                if i == s[0..i.len()] {
                    result += 1;
                }
            }
        }
        
        result
    }
}