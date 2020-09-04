- [257. 二叉树的所有路径](#257-二叉树的所有路径)
  - [官方题解](#官方题解)
    - [方法一：深度优先搜索](#方法一深度优先搜索)
    - [方法二：广度优先搜索](#方法二广度优先搜索)


------------------------------

# 257. 二叉树的所有路径

给定一个二叉树，返回所有从根节点到叶子节点的路径。

说明: 叶子节点是指没有子节点的节点。

示例:

```
输入:

   1
 /   \
2     3
 \
  5

输出: ["1->2->5", "1->3"]

解释: 所有根节点到叶子节点的路径为: 1->2->5, 1->3
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/binary-tree-paths
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/binary-tree-paths/solution/er-cha-shu-de-suo-you-lu-jing-by-leetcode-solution/

### 方法一：深度优先搜索

**思路与算法**

最直观的方法是使用深度优先搜索。在深度优先搜索遍历二叉树时，我们需要考虑当前的节点以及它的孩子节点。

- 如果当前节点不是叶子节点，则在当前的路径末尾添加该节点，并继续递归遍历该节点的每一个孩子节点。
- 如果当前节点是叶子节点，则在当前路径末尾添加该节点后我们就得到了一条从根节点到叶子节点的路径，将该路径加入到答案即可。

如此，当遍历完整棵二叉树以后我们就得到了所有从根节点到叶子节点的路径。当然，深度优先搜索也可以使用非递归的方式实现，这里不再赘述。

```go
var paths []string

func binaryTreePaths(root *TreeNode) []string {
    paths = []string{}
    constructPaths(root, "")
    return paths
}

func constructPaths(root *TreeNode, path string) {
    if root != nil {
        pathSB := path
        pathSB += strconv.Itoa(root.Val)
        if root.Left == nil && root.Right == nil {
            paths = append(paths, pathSB)
        } else {
            pathSB += "->"
            constructPaths(root.Left, pathSB)
            constructPaths(root.Right, pathSB)
        }
    }
}
```

**复杂度分析**

- 时间复杂度：$O(N^2)$，其中 N 表示节点数目。在深度优先搜索中每个节点会被访问一次且只会被访问一次，每一次会对 path 变量进行拷贝构造，时间代价为 $O(N)$，故时间复杂度为 $O(N^2)$。 
- 空间复杂度：$O(N^2)$，其中 N 表示节点数目。除答案数组外我们需要考虑递归调用的栈空间。在最坏情况下，当二叉树中每个节点只有一个孩子节点时，即整棵二叉树呈一个链状，此时递归的层数为 N，此时每一层的 path 变量的空间代价的总和为 $O(\sum_{i = 1}^{N} i) = O(N^2)$ 空间复杂度为 $O(N^2)$。最好情况下，当二叉树为平衡二叉树时，它的高度为 $\log N$，此时空间复杂度为 $O((\log {N})^2)$。



### 方法二：广度优先搜索

**思路与算法**

我们也可以用广度优先搜索来实现。我们维护一个队列，存储节点以及根到该节点的路径。一开始这个队列里只有根节点。在每一步迭代中，我们取出队列中的首节点，如果它是叶子节点，则将它对应的路径加入到答案中。如果它不是叶子节点，则将它的所有孩子节点加入到队列的末尾。当队列为空时广度优先搜索结束，我们即能得到答案。

```go
func binaryTreePaths(root *TreeNode) []string {
    paths := []string{}
    if root == nil {
        return paths
    }
    nodeQueue := []*TreeNode{}
    pathQueue := []string{}
    nodeQueue = append(nodeQueue, root)
    pathQueue = append(pathQueue, strconv.Itoa(root.Val))

    for i := 0; i < len(nodeQueue); i++ {
        node, path := nodeQueue[i], pathQueue[i]
        if node.Left == nil && node.Right == nil {
            paths = append(paths, path)
            continue
        }
        if node.Left != nil {
            nodeQueue = append(nodeQueue, node.Left)
            pathQueue = append(pathQueue, path + "->" + strconv.Itoa(node.Left.Val))
        }
        if node.Right != nil {
            nodeQueue = append(nodeQueue, node.Right)
            pathQueue = append(pathQueue, path + "->" + strconv.Itoa(node.Right.Val))
        }
    }
    return paths
}
```

**复杂度分析**

- 时间复杂度：$O(N^2)$，其中 N 表示节点数目。分析同方法一。
- 空间复杂度：$O(N^2)$，其中 N 表示节点数目。在最坏情况下，队列中会存在 N 个节点，保存字符串的队列中每个节点的最大长度为 N，故空间复杂度为 $O(N^2)$。
