struct Solution {}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let mut result = 0;
        let mut current = 0;

        for i in sentences {
            for j in i.chars() {
                if j == ' ' {
                    current += 1
                }
            }

            if current > 1 {current += 1}

            if current > result {result = current}

            current = 0
        }

        result
    }
}

fn main() {

    let arg1 = vec![
        String::from(""),
        String::from(""),
        String::from(""),
        ];

    println!("{}", Solution::most_words_found(arg1));
}
