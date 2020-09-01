struct Solution;

use std::cmp::Ordering;

struct UnionFind {
    parent: Vec<usize>, // 节点的祖先
    rank: Vec<u32>,     // 秩
    count: i32,         // 集合的数量
}

impl UnionFind {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        let mut count = 0;
        let (m, n) = (grid.len(), grid[0].len());
        let mut parent = vec![0; m * n];
        let mut rank = vec![0; m * n];
        for i in 0..m {
            for j in 0..n {
                let pos = i * n + j;
                if grid[i][j] == '1' {
                    parent[pos] = pos; // 初始时，自己是自己的祖先
                    count += 1;
                }
                rank[pos] = 0;
            }
        }
        Self {
            parent,
            rank,
            count,
        }
    }

    fn find(&mut self, pos: usize) -> usize {
        if self.parent[pos] != pos {
            // 如果 pos 不是代表，那么向上查找，将最终的结果直接赋值给 parent[pos]，压缩路径。
            self.parent[pos] = self.find(self.parent[pos]);
        }
        self.parent[pos]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                Ordering::Less => self.parent[root_x] = root_y,
                Ordering::Greater => self.parent[root_y] = root_x,
                Ordering::Equal => {
                    self.parent[root_y] = root_x;
                    self.rank[root_x] += 1; // 树高增一
                }
            }
            self.count -= 1; // 不相交集合的数量减一。
        }
    }
}

impl Solution {
    // 并查集。
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();

        let mut uf = UnionFind::new(&grid);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != '1' {
                    continue;
                }
                grid[i][j] = '0';
                // 当前位置和四个方向的合并
                let cur_pos = i * n + j;
                if i > 0 && grid[i - 1][j] == '1' {
                    uf.union(cur_pos, (i - 1) * n + j);
                }
                if j > 0 && grid[i][j - 1] == '1' {
                    uf.union(cur_pos, i * n + j - 1);
                }
                if i + 1 < m && grid[i + 1][j] == '1' {
                    uf.union(cur_pos, (i + 1) * n + j);
                }
                if j + 1 < n && grid[i][j + 1] == '1' {
                    uf.union(cur_pos, i * n + j + 1);
                }
            }
        }
        uf.count
    }

    // 失败了。
    pub fn num_islands_failed(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        let mut ans = 0;

        for x in 0..m {
            for y in 0..n {
                // if grid[x][y] == '1' {
                // 这里本来是想：如果周围已经有标记过的了，那么 ans 就不加 1 了。
                // 但是，如果周围没有标记，不代表 x,y 不和其它陆地相连，只是可能还没有扫描到。
                // if !Self::is_land_around(&grid, x, y, m, n) {
                //     ans += 1;
                // }
                // grid[x][y] = 'X';
                // }

                match grid[x][y] {
                    '1' | 'X' => {
                        ans += Self::mark_land_around(&mut grid, x, y, m, n);
                        grid[x][y] = 'X';
                    }
                    _ => {}
                }
            }
        }

        ans
    }

    fn mark_land_around(grid: &mut Vec<Vec<char>>, x: usize, y: usize, m: usize, n: usize) -> i32 {
        let mut new_land = true;
        // 如果左边和上边是 X，说明当前位置和已经扫描过的陆地是相连的。
        // 上边和左边已经扫描过了，所以可能是 0|X,不可能是 1。
        if x > 0 && grid[x - 1][y] == 'X' {
            new_land = false;
        }
        if y > 0 && grid[x][y - 1] == 'X' {
            new_land = false;
        }
        // 右边可能扫描过，也可能没有扫描过，所以可能是 0|1|X.
        if y + 1 < n {
            match grid[x][y + 1] {
                '1' => grid[x][y + 1] = 'X',
                'X' => new_land = false,
                _ => {}
            }
        }
        // 下边肯定没有扫描过，只能是 0|1.
        if x + 1 < m && grid[x + 1][y] == '1' {
            grid[x + 1][y] = 'X';
        }

        if new_land {
            1
        } else {
            0
        }
    }

    fn is_land_around(grid: &Vec<Vec<char>>, x: usize, y: usize, m: usize, n: usize) -> bool {
        if x > 0 && grid[x - 1][y] == 'X' {
            return true;
        }
        if x + 1 < m && grid[x + 1][y] == 'X' {
            return true;
        }
        if y > 0 && grid[x][y - 1] == 'X' {
            return true;
        }
        if y + 1 < n && grid[x][y + 1] == 'X' {
            return true;
        }

        false
    }

    // 深度优先遍历。
    pub fn num_islands1(mut grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        let mut ans = 0;

        for row in 0..m {
            for col in 0..n {
                // 每发现一个陆地，就把相连的都标记成 'X'。
                if grid[row][col] == '1' {
                    ans += 1;
                    Self::mark(&mut grid, row, col, m, n);
                }
            }
        }

        ans
    }

    fn mark(grid: &mut Vec<Vec<char>>, x: usize, y: usize, m: usize, n: usize) {
        if grid[x][y] != '1' {
            return;
        }
        grid[x][y] = 'X';
        // 上
        if x > 0 {
            Self::mark(grid, x - 1, y, m, n);
        }
        // 下
        if x + 1 < m {
            Self::mark(grid, x + 1, y, m, n);
        }
        // 左
        if y > 0 {
            Self::mark(grid, x, y - 1, m, n);
        }
        // 右
        if y + 1 < n {
            Self::mark(grid, x, y + 1, m, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_num_islands2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }

    #[test]
    fn test_num_islands3() {
        let grid = vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_num_islands4() {
        let grid = vec![
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['1', '1', '1', '0', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }
}
