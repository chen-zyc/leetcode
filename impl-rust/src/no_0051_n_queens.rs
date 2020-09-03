struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut ans = Vec::new();
        let mut queens = vec![-1; n as usize];
        let mut columns = HashMap::new();
        let mut diagonals1 = HashMap::new();
        let mut diagonals2 = HashMap::new();
        Self::backtrack(
            &mut queens,
            n as usize,
            0,
            &mut columns,
            &mut diagonals1,
            &mut diagonals2,
            &mut ans,
        );
        ans
    }

    fn backtrack(
        queens: &mut [isize],
        n: usize,
        row: usize,
        columns: &mut HashMap<usize, bool>,
        diagonals1: &mut HashMap<isize, bool>,
        diagonals2: &mut HashMap<usize, bool>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            ans.push(Self::generate_board(queens, n));
            return;
        }
        // i 表示第几列
        for i in 0..n {
            if columns.contains_key(&i) {
                continue;
            }
            let diagonal1 = row as isize - i as isize;
            if diagonals1.contains_key(&diagonal1) {
                continue;
            }
            let diagonal2 = row + i;
            if diagonals2.contains_key(&diagonal2) {
                continue;
            }

            queens[row] = i as isize;
            columns.insert(i, true);
            diagonals1.insert(diagonal1, true);
            diagonals2.insert(diagonal2, true);

            Self::backtrack(queens, n, row + 1, columns, diagonals1, diagonals2, ans);

            queens[row] = -1;
            columns.remove(&i);
            diagonals1.remove(&diagonal1);
            diagonals2.remove(&diagonal2);
        }
    }

    fn generate_board(queens: &[isize], n: usize) -> Vec<String> {
        // use std::iter::FromIterator;

        let mut board = Vec::with_capacity(n);
        for i in 0..n {
            // 每一行生成一个字符串
            // 皇后的位置
            let queen_pos = queens[i] as usize;
            // ...Q...
            // board.push(String::from_iter(
            //     (0..queen_pos)
            //         .map(|_| '.')
            //         .chain(std::iter::once('Q'))
            //         .chain((queen_pos + 1..n).map(|_| '.')),
            // ));
            // 这两种方式差不多啊。
            let mut s = vec!['.'; n];
            s[queen_pos] = 'Q';
            board.push(s.into_iter().collect());
        }
        board
    }
}
