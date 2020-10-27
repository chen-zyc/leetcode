package no_0144_binary_tree_preorder_traversal

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func preorderTraversal(root *TreeNode) []int {
	var ans []int

	for root != nil {
		if root.Left == nil {
			ans = append(ans, root.Val)
			root = root.Right
			continue
		}
		// 前驱节点
		prev := root.Left
		for ; prev.Right != nil && prev.Right != root; prev = prev.Right {
		}
		if prev.Right == nil {
			ans = append(ans, root.Val)
			prev.Right = root
			root = root.Left
		} else {
			// 这里 root 已经被访问过一次了，这里就不访问 root 了。
			prev.Right = nil
			root = root.Right
		}
	}

	return ans
}
