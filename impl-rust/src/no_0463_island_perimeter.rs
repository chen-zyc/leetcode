struct Solution;
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] != 1 {
                    continue;
                }
                ans += 4; // 4 条边

                // 如果上边也是岛，那么相连的两条边都从答案中去掉
                if row > 0 && grid[row - 1][col] == 1 {
                    ans -= 2;
                }
                // 如果左边是岛，那么相连的两条边去掉
                if col > 0 && grid[row][col - 1] == 1 {
                    ans -= 2;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_island_perimeter() {
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ];
        assert_eq!(Solution::island_perimeter(grid), 16);
    }
}
