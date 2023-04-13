fn main() {
    println!("{:?}", Solution::next_greater_element(
        vec![4,1,2], vec![1,3,4,2]
    ));
}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < nums1.len() {
            let mut j = 0;
            while j < nums2.len() {
                if nums1[i] == nums2[j] {
                    if j + 1 < nums2.len() {
                        let mut k = j + 1;
                        let mut trigger = result.len();
                        while k < nums2.len() {
                            if nums1[i] < nums2[k] {
                                result.push(nums2[k]);
                                break;
                            }

                            k += 1;
                        }
                        if trigger == result.len() {
                            result.push(-1);
                        } 

                        break;
                    } else {
                        result.push(-1);
                    }
                }

                j += 1;
            }

            i += 1;
        }

        result
    }
}

struct Solution {}
