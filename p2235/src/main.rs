fn main() {
    let wanna_laugh = true;
    let want_something_special = true;
    let seek_for_meaning = true;

    let inside_outtakes = String::from(
        "https://www.youtube.com/watch?v=5XWEVoI40sE"
    );

    if wanna_laugh && 
        want_something_special && 
            seek_for_meaning
    {
        println!("Click me! {}", inside_outtakes)
    }
}

struct Solution {}

impl Solution {
    pub fn sum(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

