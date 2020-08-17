- [110. 平衡二叉树](#110-平衡二叉树)
  - [题目](#题目)
  - [题解](#题解)

------------------------------

# 110. 平衡二叉树

## 题目

给定一个二叉树，判断它是否是高度平衡的二叉树。

本题中，一棵高度平衡二叉树定义为：

> 一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过1。

示例 1:

```
给定二叉树 [3,9,20,null,null,15,7]

    3
   / \
  9  20
    /  \
   15   7
返回 true 。
```

示例 2:

```
给定二叉树 [1,2,2,3,3,null,null,4,4]

       1
      / \
     2   2
    / \
   3   3
  / \
 4   4
返回 false 。
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/balanced-binary-tree
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/balanced-binary-tree/solution/ping-heng-er-cha-shu-by-leetcode-solution/

题解这里省略了，大体就是递归遍历子树，每次遍历都判断下子树是不是平衡的，如果是平衡的，就返回树的高度（左右子树的最高）。

```go
func isBalanced(root *TreeNode) bool {
    return height(root) >= 0
}

func height(root *TreeNode) int {
    if root == nil {
        return 0
    }
    leftHeight := height(root.Left)
    // 这里还可以优化，如果左子树不是平衡的，就不用再遍历右子树了。
    rightHeight := height(root.Right)
    if leftHeight == -1 || rightHeight == -1 || abs(leftHeight - rightHeight) > 1 {
        return -1
    }
    return max(leftHeight, rightHeight) + 1
}

func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}

func abs(x int) int {
    if x < 0 {
        return -1 * x
    }
    return x
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 是二叉树中的节点个数。使用自底向上的递归，每个节点的计算高度和判断是否平衡都只需要处理一次，最坏情况下需要遍历二叉树中的所有节点，因此时间复杂度是 $O(n)$。
- 空间复杂度：$O(n)$，其中 n 是二叉树中的节点个数。空间复杂度主要取决于递归调用的层数，递归调用的层数不会超过 n。
