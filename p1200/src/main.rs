fn main() {
    println!("{:?}", Solution::minimum_abs_difference(vec![4,2,1,3]));
    println!("{:?}", Solution::minimum_abs_difference(vec![1,3,6,10,15]));
    println!("{:?}", Solution::minimum_abs_difference(vec![-25,-51,-29,-23,57,-18]));
    println!("{:?}", Solution::minimum_abs_difference(vec![-12,17,-59,50,10,83,27,-79]));
}

struct Solution {}

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        let mut arr = arr;

        arr.sort();

        let mut min = i32::abs(arr[0] - arr[1]);

        for i in 1..arr.len() {
            if i32::abs(arr[i] - arr[i - 1]) < min {
                min = i32::abs(arr[i] - arr[i - 1]);
            }
        }

        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == min {
                result.push(vec![arr[i - 1], arr[i]]);
            }
        }

        result
    }
}