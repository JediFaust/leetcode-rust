struct Solution {}

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut result: Vec<String> = names.clone();
        let mut h = heights.clone();

        let mut i = 0;
        let mut k = 0;

        let mut swap = true;

        while swap {
            swap = false;
            i = 0;
            while i < h.len() - k - 1 {
                if h[i] < h[i + 1] {
                    swap = true;

                    let tmp = h[i];
                    h[i] = h[i + 1];
                    h[i + 1] = tmp;

                    let n_tmp = result[i].clone();
                    result[i] = result[i + 1].clone();
                    result[i + 1] = n_tmp;
                }

                i += 1;
            }

            k += 1;
        }

        return result;
    }
}

fn main() {
    println!("{:?}", Solution::sort_people(
            vec![String::from("n1"), String::from("n2"), String::from("n3")],
            vec![180, 165, 170]
        ));
}
