
fn main() {
    println!("{:?}", Solution::two_out_of_three(vec![0], vec![0], vec![1]));
    println!("{:?}", Solution::two_out_of_three(vec![1,1,3,2], vec![2,3], vec![3]));
    println!("{:?}", Solution::two_out_of_three(vec![1,3], vec![2,3], vec![2,1]));
    println!("{:?}", Solution::two_out_of_three(vec![1,2,2], vec![4,3,3], vec![5]));
}
struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
       let mut result = Vec::new();
       let mut data = HashSet::new();
       let mut data1 = HashSet::new();
       let mut data2 = HashSet::new();
       let mut data3 = HashSet::new();



       for i in nums1 {
          data.insert(i);
          data1.insert(i);
       }

       for i in nums2 {
          data.insert(i);
          data2.insert(i); 
       }

       for i in nums3 {
          data.insert(i);
          data3.insert(i); 
       }


       for i in data {
          if (data1.contains(&i) && data2.contains(&i))
          || (data2.contains(&i) && data3.contains(&i))
          || (data1.contains(&i) && data3.contains(&i)) {
            result.push(i);
          }
       }
       
       

       result
   }
}