fn main() {
    println!("{}", Solution::minimum_operations(vec![1,5,3,0,5]));
}

struct Solution {}

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut steps = 0;

        let mut trigger = true;

        let mut o_nums = nums;

        while trigger {
            let mut minimal = o_nums[0].clone();

            trigger = false;

            for i in &o_nums {
                if i < &minimal {
                    minimal = *i;
                }
                if *i != 0 {
                    trigger = true;
                }
            }

            if !trigger {
                break;
            }

            for i in &mut o_nums {
                if *i >= minimal {
                    *i -= minimal;
                } else {
                    *i = 0;
                }
            }

            steps += 1;
        }
        steps
    }
}