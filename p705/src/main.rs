fn main() {
    println!("Hello, world!");
}

struct Solution {}

use std::collections::HashSet;

struct MyHashSet {

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        HashSet::new()
    }
    
    fn add(&self, key: i32) {
        self.inser
    }
    
    fn remove(&self, key: i32) {
        
    }
    
    fn contains(&self, key: i32) -> bool {
        
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */