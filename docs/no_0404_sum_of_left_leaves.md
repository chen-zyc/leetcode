- [404. 左叶子之和](#404-左叶子之和)
  - [官方题解](#官方题解)
    - [方法一：深度优先搜索](#方法一深度优先搜索)
    - [方法二：广度优先搜索](#方法二广度优先搜索)

------------------------------

# 404. 左叶子之和

计算给定二叉树的所有左叶子之和。

示例：

```
    3
   / \
  9  20
    /  \
   15   7
```

在这个二叉树中，有两个左叶子，分别是 9 和 15，所以返回 24

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/sum-of-left-leaves
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

## 官方题解

> 链接：https://leetcode-cn.com/problems/sum-of-left-leaves/solution/zuo-xie-zi-zhi-he-by-leetcode-solution/

**前言**

一个节点为「左叶子」节点，当且仅当它是某个节点的左子节点，并且它是一个叶子结点。因此我们可以考虑对整棵树进行遍历，当我们遍历到节点 $\textit{node}$ 时，如果它的左子节点是一个叶子结点，那么就将它的左子节点的值累加计入答案。

遍历整棵树的方法有深度优先搜索和广度优先搜索，下面分别给出了实现代码。

### 方法一：深度优先搜索

```go
func isLeafNode(node *TreeNode) bool {
    return node.Left == nil && node.Right == nil
}

func dfs(node *TreeNode) (ans int) {
    if node.Left != nil {
        if isLeafNode(node.Left) {
            ans += node.Left.Val
        } else {
            ans += dfs(node.Left)
        }
    }
    if node.Right != nil && !isLeafNode(node.Right) {
        ans += dfs(node.Right)
    }
    return
}

func sumOfLeftLeaves(root *TreeNode) int {
    if root == nil {
        return 0
    }
    return dfs(root)
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 是树中的节点个数。
- 空间复杂度：$O(n)$。空间复杂度与深度优先搜索使用的栈的最大深度相关。在最坏的情况下，树呈现链式结构，深度为 $O(n)$，对应的空间复杂度也为 $O(n)$。

### 方法二：广度优先搜索

```go
func isLeafNode(node *TreeNode) bool {
    return node.Left == nil && node.Right == nil
}

func sumOfLeftLeaves(root *TreeNode) (ans int) {
    if root == nil {
        return
    }
    q := []*TreeNode{root}
    for len(q) > 0 {
        node := q[0]
        q = q[1:]
        if node.Left != nil {
            if isLeafNode(node.Left) {
                ans += node.Left.Val
            } else {
                q = append(q, node.Left)
            }
        }
        if node.Right != nil && !isLeafNode(node.Right) {
            q = append(q, node.Right)
        }
    }
    return
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 是树中的节点个数。
- 空间复杂度：$O(n)$。空间复杂度与广度优先搜索使用的队列需要的容量相关，为 $O(n)$。
