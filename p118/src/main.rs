use core::num;

struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut i = 0;
        let mut j = 0;

        let mut current: Vec<i32> = Vec::new();

        while i < num_rows {
            current = vec![];
            j = 1;

            while j <= i + 1 {
                current.push(j - i);
                j += 1;
            }
            
            result.push(current.clone());
            i += 1;
        }

        return result;
    }
}

fn main() {
    println!("Hello, world!");
}
