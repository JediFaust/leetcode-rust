fn main() {
    println!("{:?}", Solution::merge_similar_items(
        vec![vec![1,3],vec![2,2]], vec![vec![7,1],vec![2,2],vec![1,4]]
    ));
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut total = HashMap::new();
        let mut entries = Vec::new();
        
        for i in items1 {
            if !total.contains_key(&i[0]) {
                entries.push(i[0]);
            }
            total.insert(i[0], i[1]);
        }

        for i in items2 {
            if !total.contains_key(&i[0]) {
                entries.push(i[0]);
            }
            let item = total.entry(i[0]).or_insert(0);

            *item += i[1];
        }

        entries.sort();

        let mut result = Vec::new();

        for i in entries {
            match total.get(&i) {
                Some(v) => result.push([i, *v].to_vec()),
                None => break,
            }
        }
        

        result
    }
}