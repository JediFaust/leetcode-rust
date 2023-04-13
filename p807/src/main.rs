use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut row_max = Vec::new();
        let mut col_max = Vec::new();

        let mut i = 0;
        let mut j = 0;

        let mut max_r;
        let mut max_c;

        while i < grid.len() {
            j = 0;
            max_r = 0;
            max_c = 0;
            while j < grid.len() {
                if grid[i][j] > max_r {
                    max_r = grid[i][j];
                }

                if grid[j][i] > max_c {
                    max_c = grid[j][i];
                }

                j += 1
            }
            row_max.push(max_r);
            col_max.push(max_c);

            i += 1;
        }

        i = 0;
        j = 0;

        let mut min = row_max[i];

        while i < grid.len() {
            j = 0;
            while j < grid.len() {
                min = cmp::min(row_max[i], col_max[j]);
                result += min - grid[i][j];
                j += 1;
            }
        }

        result
    }
}

fn main() {
    println!("{}", Solution::max_increase_keeping_skyline(vec![vec![]]));
}
