- [94. 二叉树的中序遍历](#94-二叉树的中序遍历)
  - [我的思路](#我的思路)
  - [官方题解](#官方题解)
    - [方法一：递归](#方法一递归)
    - [方法二：栈](#方法二栈)
    - [方法三：Morris 中序遍历](#方法三morris-中序遍历)

------------------------------

# 94. 二叉树的中序遍历

给定一个二叉树，返回它的 **中序** 遍历。

示例:

```
输入: [1,null,2,3]
   1
    \
     2
    /
   3

输出: [1,3,2]
```

进阶: 递归算法很简单，你可以通过迭代算法完成吗？

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/binary-tree-inorder-traversal
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 我的思路

假设树是这样的：

```
   1
  / \
 2   3
    /
   4
```

如果是用栈的话，那么:
1. 先把 1 放进 栈里。`[1]`
2. 从栈里把 1 弹出，发现它有左子节点，把 1 放进去，把 2 摘掉放进栈里。`[1, 2]`
3. 弹出 2，它没有左子节点了，**输出 2**. 它也没有右子节点了，2 这颗树就访问完了。`[1]`
4. 弹出 1，左子节点已经被摘掉了，**输出 1**，1 还有右子节点，把右子节点 3 放进栈里。`[3]`
5. 弹出 3，3 有左子节点，把 3 和 左子节点都放进去。`[3, 4]`。
6. 弹出 4，4 没有子节点，**输出 4**。`[3]`
7. 弹出 3，3 也没有子节点了，**输出 3**. `[]`。
8. 栈空，结束。

总结起来就是：如果有左子节点，那么把 root 和左子节点都放进去，如果只有右子节点，那么访问 root，然后把右子节点放进去(这里是不是也可以一次把右子节点和它的左子节点都放进去？)。


我记得还有一种解法是用左右指针把树串成中序的顺序。
1. 1 有左子节点，找到 1 的左子树的最右边的那个节点 2，然后把 2 的 right 设置成 1.
2. 2 的左子节点没有，访问 2，然后 2 有右子节点（步骤 1 中设置的）。
3. 后面就不知道了，有点麻烦呀。



## 官方题解

> 链接：https://leetcode-cn.com/problems/binary-tree-inorder-traversal/solution/er-cha-shu-de-zhong-xu-bian-li-by-leetcode-solutio/

### 方法一：递归

**思路与算法**

首先我们需要了解什么是二叉树的中序遍历：按照访问左子树——根节点——右子树的方式遍历这棵树，而在访问左子树或者右子树的时候我们按照同样的方式遍历，直到遍历完整棵树。因此整个遍历过程天然具有递归的性质，我们可以直接用递归函数来模拟这一过程。

定义 `inorder(root)` 表示当前遍历到 $\textit{root}$ 节点的答案，那么按照定义，我们只要递归调用 `inorder(root.left)` 来遍历 $\textit{root}$ 节点的左子树，然后将 $\textit{root}$ 节点的值加入答案，再递归调用 `inorder(root.right)` 来遍历 $\textit{root}$ 节点的右子树即可，递归终止的条件为碰到空节点。

```go
func inorderTraversal(root *TreeNode) (res []int) {
	var inorder func(node *TreeNode)
	inorder = func(node *TreeNode) {
		if node == nil {
			return
		}
		inorder(node.Left)
		res = append(res, node.Val)
		inorder(node.Right)
	}
	inorder(root)
	return
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 为二叉树节点的个数。二叉树的遍历中每个节点会被访问一次且只会被访问一次。
- 空间复杂度：$O(n)$。空间复杂度取决于递归的栈深度，而栈深度在二叉树为一条链的情况下会达到 $O(n)$ 的级别。


### 方法二：栈

**思路与算法**

方法一的递归函数我们也可以用迭代的方式实现，两种方式是等价的，区别在于递归的时候隐式地维护了一个栈，而我们在迭代的时候需要显式地将这个栈模拟出来，其他都相同，具体实现可以看下面的代码。

```go
func inorderTraversal(root *TreeNode) (res []int) {
	stack := []*TreeNode{}
	for root != nil || len(stack) > 0 {
		for root != nil {
			stack = append(stack, root)
			root = root.Left
		}
		root = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		res = append(res, root.Val)
		root = root.Right
	}
	return
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 为二叉树节点的个数。二叉树的遍历中每个节点会被访问一次且只会被访问一次。
- 空间复杂度：$O(n)$。空间复杂度取决于栈深度，而栈深度在二叉树为一条链的情况下会达到 $O(n)$ 的级别。


### 方法三：Morris 中序遍历

**思路与算法**

Morris 遍历算法是另一种遍历二叉树的方法，它能将非递归的中序遍历空间复杂度降为 $O(1)$。

Morris 遍历算法整体步骤如下（假设当前遍历到的节点为 $x$）：

1. 如果 $x$ 无左孩子，先将 $x$ 的值加入答案数组，再访问 $x$ 的右孩子，即 $x = x.\textit{right}$。
2. 如果 $x$ 有左孩子，则找到 $x$ 左子树上最右的节点（即左子树中序遍历的最后一个节点，$x$ 在中序遍历中的前驱节点），我们记为 $\textit{predecessor}$。根据 $\textit{predecessor}$ 的右孩子是否为空，进行如下操作。
    - 如果 $\textit{predecessor}$ 的右孩子为空，则将其右孩子指向 $x$，然后访问 $x$ 的左孩子，即 $x = x.\textit{left}$。
    - 如果 $\textit{predecessor}$ 的右孩子不为空，则此时其右孩子指向 $x$，说明我们已经遍历完 $x$ 的左子树，我们将 $\textit{predecessor}$ 的右孩子置空，将 $x$ 的值加入答案数组，然后访问 $x$ 的右孩子，即 $x = x.\textit{right}$。
3. 重复上述操作，直至访问完整棵树。

![](assets/no_0094_binary_tree_inorder_traversal.gif)

其实整个过程我们就多做一步：假设当前遍历到的节点为 $x$，将 $x$ 的左子树中最右边的节点的右孩子指向 $x$，这样在左子树遍历完成后我们通过这个指向走回了 $x$，且能通过这个指向知晓我们已经遍历完成了左子树，而不用再通过栈来维护，省去了栈的空间复杂度。

```go
func inorderTraversal(root *TreeNode) (res []int) {
	for root != nil {
		if root.Left != nil {
			// predecessor 节点表示当前 root 节点向左走一步，然后一直向右走至无法走为止的节点
			predecessor := root.Left
			for predecessor.Right != nil && predecessor.Right != root {
				// 有右子树且没有设置过指向 root，则继续向右走
				predecessor = predecessor.Right
			}
			if predecessor.Right == nil {
				// 将 predecessor 的右指针指向 root，这样后面遍历完左子树 root.Left 后，就能通过这个指向回到 root
				predecessor.Right = root
				// 遍历左子树
				root = root.Left
			} else { // predecessor 的右指针已经指向了 root，则表示左子树 root.Left 已经访问完了
				res = append(res, root.Val)
				// 恢复原样
				predecessor.Right = nil
				// 遍历右子树
				root = root.Right
			}
		} else { // 没有左子树
			res = append(res, root.Val)
			// 若有右子树，则遍历右子树
			// 若没有右子树，则整颗左子树已遍历完，root 会通过之前设置的指向回到这颗子树的父节点
			root = root.Right
		}
	}
	return
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 为二叉搜索树的节点个数。Morris 遍历中**每个节点会被访问两次**，因此总时间复杂度为 $O(2n)=O(n)$。
- 空间复杂度：$O(1)$。
