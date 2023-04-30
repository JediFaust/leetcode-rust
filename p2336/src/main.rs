struct SmallestInfiniteSet {

}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    Vec<i32> set = Vec::from([1]);

    fn new() -> Self {

    }
    
    fn pop_smallest(&self) -> i32 {
        if set.length == 1 { set[0] = set[0] + 1 }
        return set[0];
    }
    
    fn add_back(&self, num: i32) {
        if set.length == 1 { }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */

fn main() {
    let obj = SmallestInfiniteSet::new();
    let ret_1: i32 = obj.pop_smallest();
    let ret_2: i32 = obj.pop_smallest();

    println!(ret_1);
    println!(ret_2);

    obj.add_back(1);
}