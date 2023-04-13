struct Solution {}

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut count = 0;

        let mut k;

        for i in words {
            count += 1;
            k = i.chars();
            for j in pref.chars() {
                match k.next() {
                    Some(v) => if v != j {
                        count -= 1;
                        break;
                    }
                    _ => {
                        count -= 1;
                        break;
                    },
                }
                
            }
        }

        return count;
    }
}

fn main() {
    println!("{}", Solution::prefix_count(
        vec![String::from("pay"),
        String::from("attention"),
        String::from("practice"),
        String::from("attend")],
        String::from("at")
    ));
}
