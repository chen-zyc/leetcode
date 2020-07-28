- [104. 二叉树的最大深度](#104-二叉树的最大深度)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：递归](#方法一递归)
    - [方法二：广度优先搜索](#方法二广度优先搜索)

------------------------------

# 104. 二叉树的最大深度

## 题目

给定一个二叉树，找出其最大深度。

二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。

说明: 叶子节点是指没有子节点的节点。

示例：

给定二叉树 `[3,9,20,null,null,15,7]`，

```
    3
   / \
  9  20
    /  \
   15   7
```

返回它的最大深度 3 。

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/maximum-depth-of-binary-tree
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/solution/er-cha-shu-de-zui-da-shen-du-by-leetcode-solution/

### 方法一：递归

**思路与算法**

如果我们知道了左子树和右子树的最大深度 l 和 r，那么该二叉树的最大深度即为

$$
\max(l,r) + 1
$$

而左子树和右子树的最大深度又可以以同样的方式进行计算。 因此我们在计算当前二叉树的最大深度时，可以先递归计算出其左子树和右子树的最大深度， 然后在 $O(1)$ 时间内计算出当前二叉树的最大深度。 递归在访问到空节点时退出。

```go
func maxDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    return max(maxDepth(root.Left), maxDepth(root.Right)) + 1
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 为二叉树节点的个数。每个节点在递归中只被遍历一次。
- 空间复杂度：$O(\textit{height})$，其中 $\textit{height}$ 表示二叉树的高度。递归函数需要栈空间，而栈空间取决于递归的深度，因此空间复杂度等价于二叉树的高度。


### 方法二：广度优先搜索

**思路与算法**

我们也可以用「广度优先搜索」的方法来解决这道题目，但我们需要对其进行一些修改，此时我们广度优先搜索的队列里存放的是「当前层的所有节点」。每次拓展下一层的时候，不同于广度优先搜索的每次只从队列里拿出一个节点，我们需要**将队列里的所有节点都拿出来**进行拓展，这样能保证每次拓展完的时候队列里存放的是当前层的所有节点，即我们是一层一层地进行拓展，最后我们用一个变量 $\textit{ans}$ 来维护拓展的次数，该二叉树的最大深度即为 $\textit{ans}$。

```go
func maxDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    queue := []*TreeNode{}
    queue = append(queue, root)
    ans := 0
    for len(queue) > 0 {
        sz := len(queue)
        // 把队列中所有节点的子节点都装进队列中。
        for sz > 0 {
            node := queue[0]
            queue = queue[1:]
            if node.Left != nil {
                queue = append(queue, node.Left)
            }
            if node.Right != nil {
                queue = append(queue, node.Right)
            }
            sz--
        }
        ans++
    }
    return ans
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 为二叉树的节点个数。与方法一同样的分析，每个节点只会被访问一次。
- 空间复杂度：此方法空间的消耗取决于队列存储的元素数量，其在最坏情况下会达到 $O(n)$。
