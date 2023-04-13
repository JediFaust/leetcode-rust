use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0, 0];

        let mut count = HashMap::new();

        for i in nums {
            let mut stm = count.get(&i);
            match stm {
                Some(v) => count.insert(&stm, Some(v + 1)),
                None => count.insert(&stm, Some(1)),
            };
            
            // count.insert(&i, count.get(&i).unwrap() + 1);
        }

        println!("{:?}", count);

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::number_of_pairs(vec![1,3,2,1,3,2,2]));
}
