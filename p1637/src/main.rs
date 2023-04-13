fn main() {
    println!("{}", Solution::max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]]));
    println!("{}", Solution::max_width_of_vertical_area(vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]]));
}

struct Solution {}

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        let mut width = Vec::new();

        for i in points {
            width.push(i[0]);
        }

        width.sort();

        for i in 1..width.len() {
            if width[i] - width[i - 1] > result {
                result = width[i] - width[i - 1];
            }
        }

        result
    }
}