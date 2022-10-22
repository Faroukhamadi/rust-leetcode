pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;

    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
        let mut count = 1;

        grid[i][j] = 0;
        for (num_i, num_j) in vec![(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            if num_i < grid.len() && num_j < grid[num_i].len() && grid[num_i][num_j] == 1 {
                count += dfs(grid, i, j);
            }
        }
        count
    }

    let mut max = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 1 {
                max = std::cmp::max(max, dfs(&mut grid, i, j));
            }
        }
    }
    max
}
