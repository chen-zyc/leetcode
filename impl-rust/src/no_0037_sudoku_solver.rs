struct Solution;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        // 9 行，每个 u16 表示一行上的数据，u16 表示对应索引的数字在这一行上已经有了。
        let mut line = vec![0_u16; 9];
        // 9 列
        let mut column = vec![0_u16; 9];
        // 3 x 3 的块
        let mut block = vec![vec![0_u16; 3]; 3];

        // 如果不是空格，就将对应的位置设置成 1.
        for (i, row) in board.iter().enumerate() {
            for (j, b) in row.iter().enumerate() {
                if *b != '.' {
                    Self::flip(
                        &mut line,
                        &mut column,
                        &mut block,
                        i,
                        j,
                        *b as u8 - '1' as u8,
                    );
                }
            }
        }

        // 先把只有一个选择的位置标记上
        loop {
            let mut modified = false;
            let m = board.len();
            let n = board[0].len();
            for i in 0..m {
                for j in 0..n {
                    let b = board[i][j];
                    if b != '.' {
                        continue;
                    }
                    // 先取反，1 就表示空格了，然后 & 0x1ff 是为了把前面的高位的 1 去掉。
                    let mask = !(line[i] | column[j] | block[i / 3][j / 3]) & 0x1ff;
                    // mask&(mask-1) 是把最右侧的 1 置 0，如果结果是 0 说明 mask 中只有一个 1，也就是只有一个选择
                    if mask > 0 && mask & (mask - 1) == 0 {
                        // 右边 0 的个数，也就是右边 1 的位置
                        let digit = mask.trailing_zeros() as u8;
                        // 将 [i][j] 的位置放上数字
                        Self::flip(&mut line, &mut column, &mut block, i, j, digit);
                        board[i][j] = (digit + '1' as u8) as char;
                        modified = true;
                    }
                }
            }
            if !modified {
                break;
            }
        }

        // 将空格收集起来
        let mut spaces = Vec::new();
        for (i, row) in board.iter().enumerate() {
            for (j, b) in row.iter().enumerate() {
                if *b == '.' {
                    spaces.push((i, j));
                }
            }
        }

        Self::dfs(&spaces, 0, &mut line, &mut column, &mut block, board);
    }

    // 将 digit 对应的比特位取反
    fn flip(
        line: &mut Vec<u16>,
        column: &mut Vec<u16>,
        block: &mut Vec<Vec<u16>>,
        i: usize,
        j: usize,
        digit: u8,
    ) {
        line[i] ^= 1 << digit;
        column[j] ^= 1 << digit;
        block[i / 3][j / 3] ^= 1 << digit;
    }

    fn dfs(
        spaces: &Vec<(usize, usize)>,
        pos: usize,
        line: &mut Vec<u16>,
        column: &mut Vec<u16>,
        block: &mut Vec<Vec<u16>>,
        board: &mut Vec<Vec<char>>,
    ) -> bool {
        if pos == spaces.len() {
            return true;
        }

        let (i, j) = spaces[pos];
        let mut mask = !(line[i] | column[j] | block[i / 3][j / 3]) & 0x1ff;
        while mask > 0 {
            let digit = mask.trailing_zeros() as u8;
            Self::flip(line, column, block, i, j, digit);
            board[i][j] = (digit + '1' as u8) as char;
            if Self::dfs(spaces, pos + 1, line, column, block, board) {
                return true;
            }
            Self::flip(line, column, block, i, j, digit);
            mask &= mask - 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_sudoku() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        let want = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        assert_eq!(board, want);
    }
}
