use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::new();
        let mut fresh_count = 0;
        let mut time = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 2 {
                    queue.push_back((i, j, 0));
                } else if grid[i][j] == 1 {
                    fresh_count += 1;
                }
            }
        }

        while let Some((i, j, t)) = queue.pop_front() {
            time = time.max(t);

            for (di, dj) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let ni = i as i32 + di;
                let nj = j as i32 + dj;

                if ni >= 0 && ni < m as i32 && nj >= 0 && nj < n as i32 {
                    let (ni, nj) = (ni as usize, nj as usize);
                    if grid[ni][nj] == 1 {
                        grid[ni][nj] = 2;
                        fresh_count -= 1;
                        queue.push_back((ni, nj, t + 1));
                    }
                }
            }
        }

        if fresh_count == 0 { time } else { -1 }
    }
}