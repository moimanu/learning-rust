pub fn min_cost(grid: &[Vec<u32>]) -> Option<u32> {
    let rows = grid.len();
    let cols = grid.first().map_or(0, Vec::len);

    if rows == 0 || cols == 0 {
        return None;
    }

    let mut dp = vec![0; cols];
    dp[0] = grid[0][0];

    for j in 1..cols {
        dp[j] = dp[j - 1] + grid[0][j];
    }

    for row in grid.iter().skip(1) {
        dp[0] += row[0];
        for j in 1..cols {
            dp[j] = row[j] + dp[j].min(dp[j - 1]);
        }
    }

    Some(dp[cols - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_none_on_empty_grid() {
        let empty: Vec<Vec<u32>> = vec![];
        assert_eq!(min_cost(&empty), None);
    }

    #[test]
    fn should_solve_example_matrix() {
        let grid = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];
        assert_eq!(min_cost(&grid), Some(7));
    }
}