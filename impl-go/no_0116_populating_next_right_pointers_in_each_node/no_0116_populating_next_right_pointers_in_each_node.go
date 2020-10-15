package no_0116_populating_next_right_pointers_in_each_node

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

	for leftmost := root; leftmost.Left != nil; leftmost = leftmost.Left {
		for cur := leftmost; cur != nil; cur = cur.Next {
			cur.Left.Next = cur.Right
			if cur.Next != nil {
				cur.Right.Next = cur.Next.Left
			}
		}
	}
	return root
}
