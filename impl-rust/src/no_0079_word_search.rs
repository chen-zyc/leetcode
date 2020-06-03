pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let m = board.len();
    if m == 0 {
        return false;
    }
    let n = board[0].len();

    let mut marked = vec![vec![false; n]; m];

    let directions = vec![
        (-1, 0), // 上
        (0, 1),  // 右
        (1, 0),  // 下
        (0, -1), // 左
    ];

    for i in 0..m {
        for j in 0..n {
            if match_word(
                &board,
                m,
                n,
                word.as_bytes(),
                0,
                i,
                j,
                &mut marked,
                &directions,
            ) {
                return true;
            }
        }
    }

    false
}

fn match_word(
    board: &Vec<Vec<char>>,
    m: usize,
    n: usize,
    word: &[u8],
    word_idx: usize,
    board_x: usize,
    board_y: usize,
    marked: &mut Vec<Vec<bool>>,
    directions: &Vec<(isize, isize)>,
) -> bool {
    if board[board_x][board_y] as u8 != word[word_idx] {
        return false;
    }
    if word_idx == word.len() - 1 {
        // 匹配完了
        return true;
    }

    marked[board_x][board_y] = true;
    // 当前匹配上了，再匹配下一个，可选的是相邻的四个位置
    for (dx, dy) in directions {
        let (x, y) = (board_x as isize + *dx, board_y as isize + *dy);
        if 0 <= x && x < m as isize && 0 <= y && y < n as isize {
            let (x, y) = (x as usize, y as usize);
            if !marked[x][y]
                && match_word(board, m, n, word, word_idx + 1, x, y, marked, directions)
            {
                return true;
            }
        }
    }
    // 四个方向都没有匹配的上，退回到上一步
    marked[board_x][board_y] = false;
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        assert_eq!(exist(board.clone(), "ABCCED".to_owned()), true);
        assert_eq!(exist(board.clone(), "SEE".to_owned()), true);
        assert_eq!(exist(board.clone(), "ABCB".to_owned()), false);
    }
}
