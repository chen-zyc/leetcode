- [144. 二叉树的前序遍历](#144-二叉树的前序遍历)
  - [官方题解](#官方题解)
    - [方法一：递归](#方法一递归)
    - [方法二：迭代](#方法二迭代)
    - [方法三：Morris 遍历](#方法三morris-遍历)

------------------------------

# 144. 二叉树的前序遍历

给定一个二叉树，返回它的 **前序** 遍历。

示例:

```
输入: [1,null,2,3]  
   1
    \
     2
    /
   3 

输出: [1,2,3]
```

进阶: 递归算法很简单，你可以通过迭代算法完成吗？

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/binary-tree-preorder-traversal
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/binary-tree-preorder-traversal/solution/er-cha-shu-de-qian-xu-bian-li-by-leetcode-solution/

### 方法一：递归

思路略。

```go
func preorderTraversal(root *TreeNode) (vals []int) {
    var preorder func(*TreeNode)
    preorder = func(node *TreeNode) {
        if node == nil {
            return
        }
        vals = append(vals, node.Val)
        preorder(node.Left)
        preorder(node.Right)
    }
    preorder(root)
    return
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 是二叉树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：$O(n)$，为递归过程中栈的开销，平均情况下为 $O(\log n)$，最坏情况下树呈现链状，为 $O(n)$。

### 方法二：迭代

```go
func preorderTraversal(root *TreeNode) (vals []int) {
    stack := []*TreeNode{}
    node := root
    for node != nil || len(stack) > 0 {
        for node != nil {
            vals = append(vals, node.Val)
            stack = append(stack, node)
            node = node.Left
        }
        node = stack[len(stack)-1].Right
        stack = stack[:len(stack)-1]
    }
    return
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 是二叉树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：$O(n)$，为迭代过程中显式栈的开销，平均情况下为 $O(\log n)$，最坏情况下树呈现链状，为 $O(n)$。

### 方法三：Morris 遍历

有一种巧妙的方法可以在线性时间内，只占用常数空间来实现前序遍历。这种方法由 J. H. Morris 在 1979 年的论文「Traversing Binary Trees Simply and Cheaply」中首次提出，因此被称为 Morris 遍历。

Morris 遍历的核心思想是利用树的大量空闲指针，实现空间开销的极限缩减。其前序遍历规则总结如下：

1. 新建临时节点，令该节点为 root；
2. 如果当前节点的左子节点为空，将当前节点加入答案，并遍历当前节点的右子节点；
3. 如果当前节点的左子节点不为空，在当前节点的左子树中找到当前节点在中序遍历下的前驱节点：
    - 如果前驱节点的右子节点为空，将前驱节点的右子节点设置为当前节点。然后将当前节点加入答案，并将前驱节点的右子节点更新为当前节点。
    - 如果前驱节点的右子节点为当前节点，将它的右子节点重新设为空。当前节点更新为当前节点的右子节点。
4. 重复步骤 2 和步骤 3，直到遍历结束。

这样我们利用 Morris 遍历的方法，前序遍历该二叉树，即可实现线性时间与常数空间的遍历。

```go
func preorderTraversal(root *TreeNode) (vals []int) {
    var p1, p2 *TreeNode = root, nil
    for p1 != nil {
        p2 = p1.Left
        if p2 != nil {
            for p2.Right != nil && p2.Right != p1 {
                p2 = p2.Right
            }
            if p2.Right == nil {
                vals = append(vals, p1.Val)
                p2.Right = p1
                p1 = p1.Left
                continue
            }
            p2.Right = nil
        } else {
            vals = append(vals, p1.Val)
        }
        p1 = p1.Right
    }
    return
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 是二叉树的节点数。没有左子树的节点只被访问一次，有左子树的节点被访问两次。
- 空间复杂度：$O(1)$。只操作已经存在的指针（树的空闲指针），因此只需要常数的额外空间。
