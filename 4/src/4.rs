struct Solution {}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let result: f64;
        let mut total_vec: Vec<i32> = Vec::new();
        let total_len: usize = nums1.len() + nums2.len();

        let first_i: usize;
        let second_i: usize;

        if total_len % 2 == 0 {
            second_i = total_len / 2;
            first_i = second_i - 1;
        } else {
            first_i = (total_len - 1) / 2;
            second_i = first_i;
        }

        let mut first = nums1.len() > 0;
        let mut second = nums2.len() > 0;

        let mut i: usize = 0;
        let mut j: usize = 0;

        loop {
            if !first {
                total_vec.push(nums2[j]);

                if j == nums2.len() - 1 { second = false } else { j += 1 }
            } else if !second {
                total_vec.push(nums1[i]);

                if i == nums1.len() - 1 { first = false } else { i += 1 }
            } else {
                let is_first = nums1[i] <= nums2[j];

                if is_first {
                    total_vec.push(nums1[i]);

                    if i == nums1.len() - 1 { first = false } else { i += 1 }
                } else {
                    total_vec.push(nums2[j]);

                    if j == nums2.len() - 1 { second = false } else { j += 1 }
                }
            }

            if !first && !second { break }
        }

        result = f64::from(total_vec[first_i] + total_vec[second_i]) / 2.0;

        println!("{:?}", total_vec);

        result
    }
}

fn main() {
    println!("{:?}", Solution::find_median_sorted_arrays(vec![1, 3, 6], vec![]));

    println!("{:?}", Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));

}