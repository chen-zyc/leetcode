struct Solution;
impl Solution {
    // bfs
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let (m, n) = (board.len(), board[0].len());
        let mut queue = Vec::new();
        // 上下边界
        for i in 0..m {
            if board[i][0] == 'O' {
                queue.push((i, 0));
            }
            if board[i][n - 1] == 'O' {
                queue.push((i, n - 1));
            }
        }
        // 左右边界
        for i in 1..(n - 1) {
            if board[0][i] == 'O' {
                queue.push((0, i));
            }
            if board[m - 1][i] == 'O' {
                queue.push((m - 1, i));
            }
        }
        // 标记
        while !queue.is_empty() {
            if let Some((x, y)) = queue.pop() {
                board[x][y] = 'A';
                // 检查四个方向
                if x > 0 && board[x - 1][y] == 'O' {
                    queue.push((x - 1, y));
                }
                if y + 1 < n && board[x][y + 1] == 'O' {
                    queue.push((x, y + 1));
                }
                if y > 0 && board[x][y - 1] == 'O' {
                    queue.push((x, y - 1));
                }
                if x + 1 < m && board[x + 1][y] == 'O' {
                    queue.push((x + 1, y));
                }
            }
        }
        // 修改
        for x in 0..m {
            for y in 0..n {
                match board[x][y] {
                    'A' => board[x][y] = 'O',
                    'O' => board[x][y] = 'X',
                    _ => {}
                }
            }
        }
    }

    // dfs
    pub fn solve1(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }
        let (m, n) = (board.len(), board[0].len());
        // 检查左右边界
        for i in 0..m {
            Self::dfs(board, i, 0, m, n);
            Self::dfs(board, i, n - 1, m, n);
        }
        // 检查上下边界
        for i in 1..(n - 1) {
            Self::dfs(board, 0, i, m, n);
            Self::dfs(board, m - 1, i, m, n);
        }
        // 把被包围的改成 'X'，没有被包围的改成 'O'
        for x in 0..m {
            for y in 0..n {
                match board[x][y] {
                    'A' => board[x][y] = 'O',
                    'O' => board[x][y] = 'X',
                    _ => {}
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, x: usize, y: usize, m: usize, n: usize) {
        if board[x][y] != 'O' {
            return;
        }
        board[x][y] = 'A';
        // 左边相邻
        if x > 0 {
            Self::dfs(board, x - 1, y, m, n);
        }
        // 右边相邻
        if y + 1 < n {
            Self::dfs(board, x, y + 1, m, n);
        }
        // 上边相邻
        if y > 0 {
            Self::dfs(board, x, y - 1, m, n);
        }
        // 下边相邻
        if x + 1 < m {
            Self::dfs(board, x + 1, y, m, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);

        let want = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        assert_eq!(want, board);
    }
}
