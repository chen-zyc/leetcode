use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    // 50. Pow(x, n)
    // https://leetcode-cn.com/problems/powx-n/
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n < 0 {
            1.0 / Solution::my_pow_helper2(x, -(n as i64))
        } else {
            Solution::my_pow_helper2(x, n as i64)
        }
    }

    fn my_pow_helper1(x: f64, n: i32) -> f64 {
        // 方法1：快速幂 + 递归
        if n == 0 {
            return 1.0;
        }
        let y = Solution::my_pow_helper1(x, n / 2);
        if n & 1 == 0 {
            // 偶数
            y * y
        } else {
            // 奇数
            y * y * x
        }
    }

    // n 为什么是 i64?
    // 如果 n 是负数，而且很大，那么当执行 -n 时可能会溢出。
    fn my_pow_helper2(mut x: f64, mut n: i64) -> f64 {
        // 方法2：快速幂 + 迭代
        let mut ans = 1.0;
        // 拆分成二进制
        while n > 0 {
            if n & 1 == 1 {
                // 当前二进制为 1，把 x 累积到答案中。
                ans *= x;
            }
            // 累积
            x *= x;
            n >>= 1;
        }

        ans
    }

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
        let mut stack = vec![0]; // 前面的哨兵。

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

    /// 88. 合并两个有序数组
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // 题解中的方法。先把最大的那个安排好，而不是先安排最小的那个。

        let (mut i, mut j, mut idx) = (m-1, n-1, nums1.len());
        while i >= 0 && j >= 0 {
            // 让 idx 始终保持在后一个位置，这样是为了避免出现 <0 的情况。
            idx -= 1;
            if nums1[i as usize] > nums2[j as usize] {
                nums1[idx] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[idx] = nums2[j as usize];
                j -= 1;
            }
        }
        // 处理剩下的部分。
        // 如果 i >= 0，就不需要处理了，因为 nums1[..=i] 是有序的，且在 nums1 的前面。
        if j >= 0 {
            nums1[..=j as usize].copy_from_slice(&nums2[..=j as usize]);
        }
    }
    pub fn merge_1(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        // 先把 nums1 里的 m 个元素移到后面去。
        nums1.rotate_left(m);

        // idx1 的偏移量是 n，因为前面有 n 个 0.
        // idx 是要存放的位置。
        let (mut idx1, mut idx2, mut idx) = (0_usize, 0_usize, 0_usize);
        while idx1 < m && idx2 < n {
            if nums1[idx1 + n] <= nums2[idx2] {
                nums1[idx] = nums1[idx1 + n];
                idx += 1;
                idx1 += 1;
            } else {
                nums1[idx] = nums2[idx2];
                idx += 1;
                idx2 += 1;
            }
        }
        // 把剩下的整理一下
        while idx1 < m {
            nums1[idx] = nums1[idx1 + n];
            idx += 1;
            idx1 += 1;
        }
        while idx2 < n {
            nums1[idx] = nums2[idx2];
            idx += 1;
            idx2 += 1;
        }
    }
    pub fn merge_failed(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        // 当前要比较的两个数组的下标。
        let (mut idx1, mut idx2) = (0_usize, 0_usize);
        // 下标都在合理范围内才比较。
        while idx1 < m && idx2 < n {
            match nums1[idx1].cmp(&nums2[idx2]) {
                // 第一个元素比较小，它的位置是正确的，不用交换，只移动下标。
                std::cmp::Ordering::Less => {
                    idx1 += 1;
                }
                // 第二个元素比较小，把它移到 nums1 里，把 nums1 里的那个移动到 nums2 里。
                // WRONG: 移到 nums2 里之后，nums2 就不是有序的了！！！
                std::cmp::Ordering::Greater => {
                    std::mem::swap(&mut nums1[idx1], &mut nums2[idx2]);
                    idx1 += 1; // idx2 就不用移动了。
                }
                // 两者相等的话就当作是第一个比较小吧。
                std::cmp::Ordering::Equal => {
                    idx1 += 1;
                }
            }
        }
        // 如果 nums1 里还剩下元素，不需要管，因为它们都是有序的。
        // 如果 nums2 里还剩下元素，需要移到 nums1 里。
        while idx2 < n {
            nums1[idx1] = nums2[idx2];
            idx1 += 1;
            idx2 += 1;
        }
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
    use crate::common::tree_node;

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
        assert!(Solution::is_valid_bst(root));

        let root = tree_node(TreeNode {
            val: 5,
            left: tree_node(TreeNode::new(1)),
            right: tree_node(TreeNode {
                val: 4,
                left: tree_node(TreeNode::new(3)),
                right: tree_node(TreeNode::new(6)),
            }),
        });
        assert!(!Solution::is_valid_bst(root));

        // 最大值。
        let root = tree_node(TreeNode::new(2147483647));
        assert!(Solution::is_valid_bst(root));
    }

    #[test]
    fn test_my_pow() {
        assert!((Solution::my_pow(2.0, 10) - 1024.0).abs() < 1e-10);
        assert!((Solution::my_pow(2.1, 3) - 9.261).abs() < 1e-10);
        assert!((Solution::my_pow(2.0, -2) - 0.25).abs() < 1e-10);
        assert!((Solution::my_pow(2.0, -2147483648) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);

        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);

        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);

        // 第一次出错的用例
        let mut nums1 = vec![4, 5, 6, 0, 0, 0];
        let mut nums2 = vec![1, 2, 3];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
