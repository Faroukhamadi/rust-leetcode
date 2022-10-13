pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid_copy = grid.clone();
    if grid.len() == 0 {
        return 0;
    }
    let mut count = 0;
    for i in 0..grid_copy.len() {
        for j in 0..grid_copy[0].len() {
            if grid_copy[i][j] == '1' {
                dfs(&mut grid_copy, i, j);
                count += 1;
            }
        }
    }
    count
}

fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
    // is rust's compiler smarter than me ?
    // the answer is definitely YESSSS
    if i >= grid.len() || j >= grid[0].len() || grid[i][j] != '1' {
        return;
    }
    grid[i][j] = ' ';
    dfs(grid, i + 1, j);
    dfs(grid, i - 1, j);
    dfs(grid, i, j + 1);
    dfs(grid, i, j - 1);
}
