use crate::common::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    /// 101. 对称二叉树
    /// https://leetcode-cn.com/problems/symmetric-tree/
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        root.map(|r| {
            let (left, right) = (r.borrow().left.clone(), r.borrow().right.clone());
            Self::is_symmetric_helper(&left, &right)
        })
        .unwrap_or(false)
    }
    /// 这种写法有点像 Go。
    fn is_symmetric_helper_1(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() {
            return right.is_none();
        }
        if right.is_none() {
            return left.is_none();
        }
        let (left, right) = (left.unwrap(), right.unwrap());
        // 如果两个节点的值不一样，直接返回 false 就可以了，剩下的就不用比较了。
        if left.borrow().val != right.borrow().val {
            return false;
        }

        // 左边和右边比较，这样才是对称的。
        let (left, right) = (left.borrow(), right.borrow());
        Self::is_symmetric_helper_1(left.left.clone(), right.right.clone())
            && Self::is_symmetric_helper_1(left.right.clone(), right.left.clone())
    }
    /// 使用 match，这种有 Rust 味了。
    fn is_symmetric_helper(
        left: &Option<Rc<RefCell<TreeNode>>>,
        right: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                let (left, right) = (left.borrow(), right.borrow());
                left.val == right.val
                    && Self::is_symmetric_helper(&left.left, &right.right)
                    && Self::is_symmetric_helper(&left.right, &right.left)
            }
            // 其中只有一个是 None，那么就不对称了。
            _ => false,
        }
    }

    /// 121. 买卖股票的最佳时机
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (_, max) = prices
            .into_iter()
            // .0 表示 n 之前的最小值。
            // .1 表示在 n 卖出时能获得的最大利润。
            .fold((i32::MAX, 0), |(min, max), n| {
                (min.min(n), max.max(n - min))
            });
        max
    }
    pub fn max_profit_1(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }
        let mut it = prices.into_iter();
        // 这里可以先把 min 设置成最大值，这样就不用先获取一次值了。
        let mut min = it.next().unwrap();
        let mut max = 0;

        for v in it {
            // 当在 min 买入，在 v 卖出时，能获得的最大利润。
            max = max.max(v - min);
            min = min.min(v);
        }
        max
    }

    /// 122. 买卖股票的最佳时机 II
    pub fn max_profit_2(prices: Vec<i32>) -> i32 {
        // 题解中还有贪心算法，我这里是为了练习动态规划。

        // [0] 表示在第 i 天不持有股票时，最大利润。
        // [1] 表示在第 i 天持有股票时，最大利润。
        let mut profit = [0, -prices[0]]; // prices.len >= 1
        let mut tmp;
        for price in prices {
            // 如果这一天不持有股票：如果昨天也没有持有股票，今天的利润就是昨天的利润；如果昨天持有股票，今天的利润就是把昨天的股票卖出的价格加上昨天的利润。
            // 注意：昨天的利润已经把买股票时的价钱减去了，所以这里不需要再减去。
            // profit[0].max(profit[1]+price);
            // 如果这一天持有股票：如果昨天没有股票，今天就得买入股票；如果昨天持有股票，今天就不需要动。
            // profit[1].max(profit[0] - price);
            tmp = profit[0];
            profit[0] = tmp.max(profit[1] + price);
            profit[1] = profit[1].max(tmp - price);
        }
        // 清仓总比不清仓要赚的多。
        profit[0]
    }

    /// 153. 寻找旋转排序数组中的最小值
    /// https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < nums[right] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        nums[left]
    }

    // 154. 寻找旋转排序数组中的最小值 II
    // https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array-ii/
    pub fn find_min_2(nums: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&nums[right]) {
                std::cmp::Ordering::Less => right = mid,
                std::cmp::Ordering::Greater => left = mid + 1,
                std::cmp::Ordering::Equal => right -= 1,
            }
        }
        nums[left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tree_node;

    #[test]
    fn test_is_symmetric() {
        let root = tree_node(TreeNode {
            val: 1,
            left: tree_node(TreeNode {
                val: 2,
                left: tree_node(TreeNode::new(3)),
                right: tree_node(TreeNode::new(4)),
            }),
            right: tree_node(TreeNode {
                val: 2,
                left: tree_node(TreeNode::new(4)),
                right: tree_node(TreeNode::new(3)),
            }),
        });
        assert!(Solution::is_symmetric(root));

        let root = tree_node(TreeNode {
            val: 1,
            left: tree_node(TreeNode {
                val: 2,
                left: None,
                right: tree_node(TreeNode::new(3)),
            }),
            right: tree_node(TreeNode {
                val: 2,
                left: None,
                right: tree_node(TreeNode::new(3)),
            }),
        });
        assert!(!Solution::is_symmetric(root));
    }

    #[test]
    fn test_find_min() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
    }

    #[test]
    fn test_find_min_2() {
        assert_eq!(Solution::find_min_2(vec![1, 3, 5]), 1);
        assert_eq!(Solution::find_min_2(vec![2, 2, 2, 0, 1]), 0);
    }

    #[test]
    fn test_max_profit() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_2() {
        assert_eq!(Solution::max_profit_2(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(Solution::max_profit_2(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit_2(vec![7, 6, 4, 3, 1]), 0);
    }
}
