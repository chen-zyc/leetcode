- [226. 翻转二叉树](#226-翻转二叉树)
  - [官方题解](#官方题解)
    - [方法一：递归](#方法一递归)

------------------------------

# 226. 翻转二叉树

翻转一棵二叉树。

示例：

输入：

```
     4
   /   \
  2     7
 / \   / \
1   3 6   9
```

输出：

```
     4
   /   \
  7     2
 / \   / \
9   6 3   1
```

备注: 这个问题是受到 Max Howell 的 [原问题](https://twitter.com/mxcl/status/608682016205344768) 启发的 ：

> 谷歌：我们90％的工程师使用您编写的软件(Homebrew)，但是您却无法在面试时在白板上写出翻转二叉树这道题，这太糟糕了。


- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/invert-binary-tree
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/invert-binary-tree/solution/fan-zhuan-er-cha-shu-by-leetcode-solution/

### 方法一：递归

**思路与算法**

这是一道很经典的二叉树问题。显然，我们从根节点开始，递归地对树进行遍历，并从叶子结点先开始翻转。如果当前遍历到的节点 $\textit{root}$ 的左右两棵子树都已经翻转，那么我们只需要交换两棵子树的位置，即可完成以 $\textit{root}$ 为根节点的整棵子树的翻转。

```go
func invertTree(root *TreeNode) *TreeNode {
    if root == nil {
        return nil
    }
    left := invertTree(root.Left)
    right := invertTree(root.Right)
    root.Left = right
    root.Right = left
    return root
}
```

**复杂度分析**

- 时间复杂度：$O(N)$，其中 N 为二叉树节点的数目。我们会遍历二叉树中的每一个节点，对每个节点而言，我们在常数时间内交换其两棵子树。
- 空间复杂度：$O(N)$。使用的空间由递归栈的深度决定，它等于当前节点在二叉树中的高度。在平均情况下，二叉树的高度与节点个数为对数关系，即 $O(\log N)$。而在最坏情况下，树形成链状，空间复杂度为 $O(N)$。
