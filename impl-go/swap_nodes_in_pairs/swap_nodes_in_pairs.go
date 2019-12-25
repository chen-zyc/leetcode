package swap_nodes_in_pairs

type ListNode struct {
	Val  int
	Next *ListNode
}

func swapPairs(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	newHead := &ListNode{Next: head}
	cur := newHead
	var first, second *ListNode
	for cur.Next != nil && cur.Next.Next != nil {
		first, second = cur.Next, cur.Next.Next
		// 重定向 first -> second
		first.Next = second.Next
		// 重定向 second -> third
		second.Next = first
		// 重定向 cur.Next
		cur.Next = second
		// cur 后移
		cur = first
	}
	return newHead.Next
}
