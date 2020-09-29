- [145. 二叉树的后序遍历](#145-二叉树的后序遍历)
  - [官方题解](#官方题解)
    - [方法一：递归](#方法一递归)
    - [方法二：迭代](#方法二迭代)
    - [方法三：Morris 遍历](#方法三morris-遍历)


------------------------------

# 145. 二叉树的后序遍历

给定一个二叉树，返回它的 **后序** 遍历。

示例:

```
输入: [1,null,2,3]  
   1
    \
     2
    /
   3 

输出: [3,2,1]
```

进阶: 递归算法很简单，你可以通过迭代算法完成吗？



- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/binary-tree-postorder-traversal
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。



## 官方题解

> 链接：https://leetcode-cn.com/problems/binary-tree-postorder-traversal/solution/er-cha-shu-de-hou-xu-bian-li-by-leetcode-solution/

### 方法一：递归

首先我们需要了解什么是二叉树的后序遍历：按照访问左子树——右子树——根节点的方式遍历这棵树，而在访问左子树或者右子树的时候，我们按照同样的方式遍历，直到遍历完整棵树。因此整个遍历过程天然具有递归的性质，我们可以直接用递归函数来模拟这一过程。

定义 `postorder(root)` 表示当前遍历到 root 节点的答案。按照定义，我们只要递归调用 `postorder(root->left)` 来遍历 root 节点的左子树，然后递归调用 `postorder(root->right)` 来遍历 root 节点的右子树，最后将 root 节点的值加入答案即可，递归终止的条件为碰到空节点。

```go
func postorderTraversal(root *TreeNode) (res []int) {
    var postorder func(*TreeNode)
    postorder = func(node *TreeNode) {
        if node == nil {
            return
        }
        postorder(node.Left)
        postorder(node.Right)
        res = append(res, node.Val)
    }
    postorder(root)
    return
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 是二叉搜索树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：$O(n)$，为递归过程中栈的开销，平均情况下为 $O(\log n)$，最坏情况下树呈现链状，为 $O(n)$。


### 方法二：迭代

我们也可以用迭代的方式实现方法一的递归函数，两种方式是等价的，区别在于递归的时候隐式地维护了一个栈，而我们在迭代的时候需要显式地将这个栈模拟出来，其余的实现与细节都相同，具体可以参考下面的代码。

```go
func postorderTraversal(root *TreeNode) (res []int) {
    stk := []*TreeNode{}
    var prev *TreeNode // 上一个访问的节点
    for root != nil || len(stk) > 0 {
        for root != nil {
            stk = append(stk, root)
            root = root.Left
        }
        root = stk[len(stk)-1]
        stk = stk[:len(stk)-1]
        if root.Right == nil || root.Right == prev {
            // 2. 如果没有右子节点，或者右子节点已经访问完了，那么可以访问 root 了。
            res = append(res, root.Val)
            prev = root
            root = nil
        } else {
            // 1. 如果有右子节点，那么要把 root 再放回去，先去访问右子节点。
            stk = append(stk, root)
            root = root.Right
        }
    }
    return
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 是二叉搜索树的节点数。每一个节点恰好被遍历一次。
- 空间复杂度：$O(n)$，为迭代过程中显式栈的开销，平均情况下为 $O(\log n)$，最坏情况下树呈现链状，为 $O(n)$。

### 方法三：Morris 遍历

有一种巧妙的方法可以在线性时间内，只占用常数空间来实现后序遍历。这种方法由 J. H. Morris 在 1979 年的论文「Traversing Binary Trees Simply and Cheaply」中首次提出，因此被称为 Morris 遍历。

Morris 遍历的核心思想是利用树的大量空闲指针，实现空间开销的极限缩减。其后序遍历规则总结如下：

1. 新建临时节点，令该节点为 root；
2. 如果当前节点的左子节点为空，则遍历当前节点的右子节点；
3. 如果当前节点的左子节点不为空，在当前节点的左子树中找到当前节点在中序遍历下的前驱节点；
    - 如果前驱节点的右子节点为空，将前驱节点的右子节点设置为当前节点，当前节点更新为当前节点的左子节点。
    - 如果前驱节点的右子节点为当前节点，将它的右子节点重新设为空。倒序输出从当前节点的左子节点到该前驱节点这条路径上的所有节点。当前节点更新为当前节点的右子节点。
4. 重复步骤 2 和步骤 3，直到遍历结束。

这样我们利用 Morris 遍历的方法，后序遍历该二叉搜索树，即可实现线性时间与常数空间的遍历。

```go
func postorderTraversal(root *TreeNode) (res []int) {
    addPath := func(node *TreeNode) {
        path := []int{}
        for ; node != nil; node = node.Right {
            path = append(path, node.Val)
        }
        for i := len(path) - 1; i >= 0; i-- {
            res = append(res, path[i])
        }
    }

    p1 := root
    for p1 != nil {
        if p2 := p1.Left; p2 != nil {
            for p2.Right != nil && p2.Right != p1 {
                p2 = p2.Right
            }
            if p2.Right == nil {
                p2.Right = p1
                p1 = p1.Left
                continue
            }
            p2.Right = nil
            addPath(p1.Left)
        }
        p1 = p1.Right
    }
    addPath(root)
    return
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 是二叉树的节点数。没有左子树的节点只被访问一次，有左子树的节点被访问两次。
- 空间复杂度：$O(1)$。只操作已经存在的指针（树的空闲指针），因此只需要常数的额外空间。
