use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

use crate::common::tree_node;

struct Solution;
impl Solution {
    // 84. 柱状图中最大的矩形
    // https://leetcode-cn.com/problems/largest-rectangle-in-histogram/
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        // let (mut p1, mut p2) = (0, heights.len());
        // let mut largest = 0;
        // while p2 > p1 {
        //     // 这种计算是不对的，因为中间可能有凹点。
        //     let area = (p2 - p1) as i32 * heights[p1].min(heights[p2 - 1]);
        //     if area > largest {
        //         largest = area;
        //     }
        //     if heights[p2 - 1] < heights[p1] {
        //         p2 -= 1;
        //     } else {
        //         p1 += 1;
        //     }
        // }
        // largest

        if heights.is_empty() {
            return 0;
        }

        // ======= 方法一：在栈中保存下标和高度。
        // // 在后面加入哨兵。
        // let heights_iter = heights.into_iter().chain(0..1);
        //
        // let mut largest = 0;
        // // (下标，高度)
        // let mut stack = vec![];
        // // 前面的哨兵，下标一定是 -1，不然计算宽度时就会不对了。
        // // 这种保存了高度的做法效率可能不高。
        // stack.push((-1_i32, 0));
        //
        // for (i, height) in heights_iter.enumerate() {
        //     while height < stack.last().unwrap().1 {
        //         // 找到一个比当前下标的高度大的。
        //         // 计算它的面积。
        //         let prev = stack.pop().unwrap();
        //         // 宽度是 prev 前面那个和当前下标之间的宽度。
        //         let area = (i as i32 - stack.last().unwrap().0 - 1) * prev.1;
        //         largest = largest.max(area);
        //     }
        //     stack.push((i as i32, height));
        // }

        // ======= 方法二：下面的实现是只在后面加哨兵，没有在前面加，但是编译不过。
        // 在后面加入哨兵。
        // let heights: Vec<_> = heights.into_iter().chain(0..1).collect();
        // let mut largest = 0;
        // let mut stack = vec![];
        // // 前面的哨兵，下标一定是 -1，不然计算宽度时就会不对了。
        // stack.push(-1_isize);
        //
        // for (i, &height) in heights.iter().enumerate() {
        //     // ERROR: 如果 stack.last == -1，那么这里就会 panic.
        //     while height < heights[*stack.last().unwrap() as usize] {
        //         // 找到一个比当前下标的高度大的。
        //         // 计算它的面积。
        //         let prev = stack.pop().unwrap() as usize;
        //         // 宽度是 prev 前面那个和当前下标之间的宽度。
        //         let area = (i as i32 - *stack.last().unwrap() as i32 - 1) * heights[prev];
        //         largest = largest.max(area);
        //     }
        //     stack.push(i as isize);
        // }

        // 方法三：把链转换成 Vec。速度上并没有提升多少。内在占用比方法一提升了些。
        // 在前后加哨兵。
        let heights: Vec<_> = (0..1).chain(heights.into_iter().chain(0..1)).collect();
        let mut largest = 0;
        let mut stack = vec![];
        // 前面的哨兵。
        stack.push(0);

        for (i, &height) in heights.iter().enumerate() {
            while height < heights[*stack.last().unwrap()] {
                // 找到一个比当前下标的高度大的。
                // 计算它的面积。
                let prev = stack.pop().unwrap();
                // 宽度是 prev 前面那个和当前下标之间的宽度。
                let area = (i - *stack.last().unwrap() - 1) as i32 * heights[prev];
                largest = largest.max(area);
            }
            // 前面的哨兵会被添加两次，不过没有关系。
            stack.push(i);
        }

        largest
    }

    // 85. 最大矩形
    // https://leetcode-cn.com/problems/maximal-rectangle/
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let (m, n) = (matrix.len(), matrix[0].len());

        // 每个点之前连续 1 的个数
        // +2 是在前后添加哨兵。
        let mut left = vec![vec![0; n]; m + 2];
        for (i, row) in matrix.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == '1' {
                    if j == 0 {
                        left[i + 1][j] = 1;
                    } else {
                        left[i + 1][j] = left[i + 1][j - 1] + 1;
                    }
                }
            }
        }

        let mut stack = Vec::new();
        let mut largest_area = 0;
        // 使用单调栈求最大矩形。
        // j 列表示 x 轴
        for j in 0..n {
            stack.clear();
            // 哨兵的位置
            stack.push(m + 1);
            // println!(
            //     "left[{}] = {:?}",
            //     j,
            //     left.iter()
            //         .map(|row| row[j].clone())
            //         .collect::<Vec<usize>>()
            // );
            for height in (0..m + 2).rev() {
                while left[height][j] < left[*stack.last().unwrap()][j] {
                    // 计算 prev 所在位置的面积。
                    let prev = stack.pop().unwrap();
                    // println!("pop stack: {}", prev);
                    // height 要比之前的小，因为 stack 里的下标是递减的。
                    let area = (stack.last().unwrap() - height - 1) * left[prev][j];
                    largest_area = largest_area.max(area);
                }
                stack.push(height);
                // println!("stack: {:?}", stack);
            }
        }

        largest_area as i32
    }

    // 98. 验证二叉搜索树
    // https://leetcode-cn.com/problems/validate-binary-search-tree/
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::is_valid_bst_helper(root, None, None)
    }

    // 使用 Option 而不是直接使用 i32，是怕 root.val 万一就是 i32::MIN 或者 i32::MAX 呢？
    // 也可以使用 i64 来代替 i32。
    fn is_valid_bst_helper(
        root: Option<Rc<RefCell<TreeNode>>>,
        lower: Option<i32>,
        upper: Option<i32>,
    ) -> bool {
        match root {
            None => true,
            Some(n) => {
                let node = n.borrow();
                // 下面这种写法参考的 https://leetcode-cn.com/problems/validate-binary-search-tree/solution/a-better-way-by-jancd/。
                lower.map_or(true, |l| node.val > l)
                    && upper.map_or(true, |u| node.val < u)
                    && Solution::is_valid_bst_helper(node.left.clone(), lower, Some(node.val))
                    && Solution::is_valid_bst_helper(node.right.clone(), Some(node.val), upper)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area() {
        assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
        assert_eq!(Solution::largest_rectangle_area(vec![1]), 1);
    }

    #[test]
    fn test_maximal_rectangle() {
        let matrix = vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ];
        assert_eq!(Solution::maximal_rectangle(matrix), 6);

        assert_eq!(Solution::maximal_rectangle(vec![]), 0);

        let matrix = vec![vec!['0']];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);

        let matrix = vec![vec!['1']];
        assert_eq!(Solution::maximal_rectangle(matrix), 1);

        let matrix = vec![vec!['0', '0']];
        assert_eq!(Solution::maximal_rectangle(matrix), 0);
    }

    #[test]
    fn test_is_valid_bst() {
        let root = tree_node(TreeNode {
            val: 2,
            left: tree_node(TreeNode::new(1)),
            right: tree_node(TreeNode::new(3)),
        });
        assert_eq!(Solution::is_valid_bst(root), true);

        let root = tree_node(TreeNode {
            val: 5,
            left: tree_node(TreeNode::new(1)),
            right: tree_node(TreeNode {
                val: 4,
                left: tree_node(TreeNode::new(3)),
                right: tree_node(TreeNode::new(6)),
            }),
        });
        assert_eq!(Solution::is_valid_bst(root), false);

        // 最大值。
        let root = tree_node(TreeNode::new(2147483647));
        assert_eq!(Solution::is_valid_bst(root), true);
    }
}
