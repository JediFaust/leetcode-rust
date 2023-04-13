fn main() {
    println!("{:?}", Solution::finding_users_active_minutes(
        vec![[1,1].to_vec(), [2,2].to_vec(), [2,3].to_vec()], 4
    ));
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut dashboard: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut result = Vec::from(vec![0; k as usize]);

        for i in logs {
            let log = dashboard.entry(i[0]).or_insert(vec![]);
            
            let mut unique = true;
            for j in log.clone() {
                if j == i[1] {
                    unique = false;
                    break;
                }
            }
            if unique {
                log.push(i[1]);
            }
        }

        for (key, val) in dashboard {
            

            if val.len() > 0 {
                result[val.len() - 1] += 1;
            }
        }

        result
    }
}


