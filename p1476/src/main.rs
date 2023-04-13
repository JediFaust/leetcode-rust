fn main() {
    let router = SubrectangleQueries::new(
        vec![
            vec![1,2,1],vec![4,3,4],vec![3,2,1],vec![1,1,1]
            ]
    );

    println!("{}", router.get_value(0, 0));
}

struct SubrectangleQueries {
    rect: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries { rect: rectangle }
    }
    
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..row2 + 1 {
            for j in col1..col2 + 1 {
                self.rect[i as usize][j as usize] = new_value;
            }
        }
    }
    
    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rect[row as usize][col as usize]
    }
}
