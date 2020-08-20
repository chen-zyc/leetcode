struct Solution;
impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        // 下一个要点击的位置
        // 可能会存在重复的位置。可以用 HashSet 过滤，或者用另一个数组来标记是否已经计算过。
        let mut stack = vec![(click[0] as usize, click[1] as usize)];
        // 八个方向
        let directions = vec![
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];
        // 行数和列数
        let (rows, cols) = (board.len() as i32, board[0].len() as i32);

        while let Some((x, y)) = stack.pop() {
            match board[x][y] {
                // 地雷，游戏结束
                'M' => {
                    board[x][y] = 'X';
                    break;
                }
                'E' => {
                    let mut mines_count = 0; // 周围的地雷数
                    let mut pos_tmp = Vec::new(); // 临时存放坐标
                    for (off_x, off_y) in &directions {
                        let (new_x, new_y) = (x as i32 + off_x, y as i32 + off_y);
                        if new_x >= 0 && new_x < rows && new_y >= 0 && new_y < cols {
                            let c = board[new_x as usize][new_y as usize];
                            // 地雷（挖出或者没有挖出）
                            if c == 'M' || c == 'X' {
                                mines_count += 1;
                            }
                            // 没有挖出的空方块或没有挖出的地雷都加入栈。
                            if mines_count == 0 && (c == 'E' || c == 'M') {
                                pos_tmp.push((new_x as usize, new_y as usize));
                            }
                        }
                    }
                    board[x][y] = if mines_count > 0 {
                        (mines_count + '0' as u8) as char
                    } else {
                        // 只有周围没有地雷时才把未点击的坐标放入栈。
                        // 使用 extend 比 append 快些。
                        stack.extend(pos_tmp);
                        'B'
                    };
                }
                _ => {}
            }
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_board1() {
        let board = vec![
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'M', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
            vec!['E', 'E', 'E', 'E', 'E'],
        ];
        let click = vec![3, 0];
        let want = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        assert_eq!(Solution::update_board(board, click), want);
    }

    #[test]
    fn test_update_board2() {
        let board = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'M', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        let click = vec![1, 2];
        let want = vec![
            vec!['B', '1', 'E', '1', 'B'],
            vec!['B', '1', 'X', '1', 'B'],
            vec!['B', '1', '1', '1', 'B'],
            vec!['B', 'B', 'B', 'B', 'B'],
        ];
        assert_eq!(Solution::update_board(board, click), want);
    }
}
