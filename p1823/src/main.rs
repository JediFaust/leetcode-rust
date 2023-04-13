fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut i = 1;

        let mut player = Vec::new();

        while i < n {
            player.push(i);

            i += 1;
        }

        i = 0;

        while player.len() > 1 {
            if i >= player.len() as i32 {
                i = i % k;
            } else {
                
            }
        }

        0
    }
}