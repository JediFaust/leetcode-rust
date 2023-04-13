struct Solution {}

impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut current: i32 = 0;

        let mut distance: f32;

        for j in queries {
            current = 0;
            for i in points.clone() {
                distance = (((
                    (i[0] - j[0]).pow(2)
                    +
                    (i[1] - j[1]).pow(2)
                ).abs() as u32) as f32).sqrt();

                if distance <= j[2] as f32 { current += 1 }
            }

            result.push(current);
        }

        return result;
    }
}

fn main() {
    println!("Hello, world!");
}
