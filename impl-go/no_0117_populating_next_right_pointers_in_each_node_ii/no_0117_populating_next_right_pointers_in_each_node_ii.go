package no_0117_populating_next_right_pointers_in_each_node_ii

import (
	"container/list"
)

// Node 二叉树的节点
type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

func connect(root *Node) *Node {
	if root == nil {
		return nil
	}

	queue := list.New()
	queue.PushBack(root)
	nodesNumCurLevel := 1

	for nodesNumCurLevel > 0 {
		var prev *Node
		for i := 0; i < nodesNumCurLevel; i++ {
			node := queue.Remove(queue.Front()).(*Node)
			if prev != nil {
				prev.Next = node
			}
			prev = node

			if node.Left != nil {
				queue.PushBack(node.Left)
			}
			if node.Right != nil {
				queue.PushBack(node.Right)
			}
		}
		nodesNumCurLevel = queue.Len()
	}

	return root
}
