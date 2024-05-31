pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }

        let (mut row, mut col) = (0, matrix[0].len() as i32 - 1);

        while row < matrix.len() && col >= 0 {
            if matrix[row][col as usize] == target {
                return true;
            } else if matrix[row][col as usize] > target {
                col -= 1;
            } else {
                row += 1;
            }
        }

        false
    }
}
