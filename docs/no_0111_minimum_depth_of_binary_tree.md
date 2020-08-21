- [111. 二叉树的最小深度](#111-二叉树的最小深度)
  - [官方题解](#官方题解)
    - [方法一：深度优先搜索](#方法一深度优先搜索)
    - [方法二：广度优先搜索](#方法二广度优先搜索)

------------------------------

# 111. 二叉树的最小深度

给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

说明: 叶子节点是指没有子节点的节点。

示例:

给定二叉树 `[3,9,20,null,null,15,7]`,

```
    3
   / \
  9  20
    /  \
   15   7
```

返回它的最小深度  2.

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/minimum-depth-of-binary-tree
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/minimum-depth-of-binary-tree/solution/er-cha-shu-de-zui-xiao-shen-du-by-leetcode-solutio/

### 方法一：深度优先搜索

**思路及解法**

首先可以想到使用深度优先搜索的方法，遍历整棵树，记录最小深度。

对于每一个非叶子节点，我们只需要分别计算其左右子树的最小叶子节点深度。这样就将一个大问题转化为了小问题，可以递归地解决该问题。

```go
func minDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    if root.Left == nil && root.Right == nil {
        return 1
    }
    minD := math.MaxInt32
    if root.Left != nil {
        minD = min(minDepth(root.Left), minD)
    }
    if root.Right != nil {
        minD = min(minDepth(root.Right), minD)
    }
    return minD + 1
}

func min(x, y int) int {
    if x < y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度：$O(N)$，其中 N 是树的节点数。对每个节点访问一次。
- 空间复杂度：$O(H)$，其中 H 是树的高度。空间复杂度主要取决于递归时栈空间的开销，最坏情况下，树呈现链状，空间复杂度为 $O(N)$。平均情况下树的高度与节点数的对数正相关，空间复杂度为 $O(\log N)$。

### 方法二：广度优先搜索

**思路及解法**

同样，我们可以想到使用广度优先搜索的方法，遍历整棵树。

当我们找到一个叶子节点时，直接返回这个叶子节点的深度。广度优先搜索的性质保证了**最先搜索到的叶子节点的深度一定最小**。

```go
func minDepth(root *TreeNode) int {
    if root == nil {
        return 0
    }
    // 一个保存节点，一个保存该节点的深度
    queue := []*TreeNode{}
    count := []int{}
    queue = append(queue, root)
    count = append(count, 1)
    for i := 0; i < len(queue); i++ {
        node := queue[i]
        depth := count[i]
        // 找到一个叶子节点后直接返回，这就是最小的深度。
        if node.Left == nil && node.Right == nil {
            return depth
        }
        if node.Left != nil {
            queue = append(queue, node.Left)
            count = append(count, depth + 1)
        }
        if node.Right != nil {
            queue = append(queue, node.Right)
            count = append(count, depth + 1)
        }
    }
    return 0
}
```

**复杂度分析**

- 时间复杂度：$O(N)$，其中 N 是树的节点数。对每个节点访问一次。
- 空间复杂度：$O(N)$，其中 N 是树的节点数。空间复杂度主要取决于队列的开销，队列中的元素个数不会超过树的节点数。
