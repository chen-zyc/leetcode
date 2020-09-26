- [113. 路径总和 II](#113-路径总和-ii)
  - [官方题解](#官方题解)
    - [方法一：深度优先搜索](#方法一深度优先搜索)
    - [方法二：广度优先搜索](#方法二广度优先搜索)

------------------------------

# 113. 路径总和 II

给定一个二叉树和一个目标和，找到所有从根节点到叶子节点路径总和等于给定目标和的路径。

说明: 叶子节点是指没有子节点的节点。

示例:
给定如下二叉树，以及目标和 sum = 22，

```
              5
             / \
            4   8
           /   / \
          11  13  4
         /  \    / \
        7    2  5   1
```

返回:

```
[
   [5,4,11,2],
   [5,8,4,5]
]
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/path-sum-ii
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/path-sum-ii/solution/lu-jing-zong-he-ii-by-leetcode-solution/

**写在前面**

注意到本题的要求是，找到**所有**满足从「根节点」到某个「叶子节点」经过的路径上的节点之和等于目标和的路径。核心思想是对树进行一次遍历，在遍历时记录从根节点到当前节点的路径和，以防止重复计算。

### 方法一：深度优先搜索

我们可以采用深度优先搜索的方式，枚举每一条从根节点到叶子节点的路径。当我们遍历到叶子节点，且此时路径和恰为目标和时，我们就找到了一条满足条件的路径。

```go
func pathSum(root *TreeNode, sum int) (ans [][]int) {
    path := []int{}
    var dfs func(*TreeNode, int)
    dfs = func(node *TreeNode, left int) {
        if node == nil {
            return
        }
        left -= node.Val
        path = append(path, node.Val)
        defer func() { path = path[:len(path)-1] }()
        if node.Left == nil && node.Right == nil && left == 0 {
            ans = append(ans, append([]int(nil), path...))
            return
        }
        dfs(node.Left, left)
        dfs(node.Right, left)
    }
    dfs(root, sum)
    return
}
```

**复杂度分析**

- 时间复杂度：$O(N^2)$，其中 NN 是树的节点数。在最坏情况下，树的上半部分为链状，下半部分为完全二叉树，并且从根节点到每一个叶子节点的路径都符合题目要求。此时，路径的数目为 $O(N)$，并且每一条路径的节点个数也为 $O(N)$，因此要将这些路径全部添加进答案中，时间复杂度为 $O(N^2)$。 
- 空间复杂度：$O(N)$，其中 N 是树的节点数。空间复杂度主要取决于栈空间的开销，栈中的元素个数不会超过树的节点数。


### 方法二：广度优先搜索

我们也可以采用广度优先搜索的方式，遍历这棵树。当我们遍历到叶子节点，且此时路径和恰为目标和时，我们就找到了一条满足条件的路径。

为了节省空间，我们使用哈希表记录树中的每一个节点的父节点。每次找到一个满足条件的节点，我们就从该节点出发不断向父节点迭代，即可还原出从根节点到当前节点的路径。

```go
type pair struct {
    node *TreeNode
    left int
}

func pathSum(root *TreeNode, sum int) (ans [][]int) {
    if root == nil {
        return
    }

    parent := map[*TreeNode]*TreeNode{}

    getPath := func(node *TreeNode) (path []int) {
        for ; node != nil; node = parent[node] {
            path = append(path, node.Val)
        }
        for i, j := 0, len(path)-1; i < j; i++ {
            path[i], path[j] = path[j], path[i]
            j--
        }
        return
    }

    queue := []pair{{root, sum}}
    for len(queue) > 0 {
        p := queue[0]
        queue = queue[1:]
        node := p.node
        left := p.left - node.Val
        if node.Left == nil && node.Right == nil {
            if left == 0 {
                ans = append(ans, getPath(node))
            }
        } else {
            if node.Left != nil {
                parent[node.Left] = node
                queue = append(queue, pair{node.Left, left})
            }
            if node.Right != nil {
                parent[node.Right] = node
                queue = append(queue, pair{node.Right, left})
            }
        }
    }

    return
}
```

**复杂度分析**

- 时间复杂度：$O(N^2)$，其中 N 是树的节点数。分析思路与方法一相同。 
- 空间复杂度：$O(N)$，其中 N 是树的节点数。空间复杂度主要取决于哈希表和队列空间的开销，哈希表需要存储除根节点外的每个节点的父节点，队列中的元素个数不会超过树的节点数。
