package remove_nth_node_from_end_of_list

type ListNode struct {
	Val  int
	Next *ListNode
}

func removeNthFromEnd(head *ListNode, n int) *ListNode {
	start := &ListNode{Val: 0, Next: head}
	slow, fast := start, start

	// 移动fast，使得slow和fast之间相隔n个
	for i := 1; i <= n+1; i++ {
		fast = fast.Next
	}

	// 移动fast到最后，保持fast和slow的间隔
	for fast != nil {
		slow = slow.Next
		fast = fast.Next
	}

	// 删除slow.Next节点
	slow.Next = slow.Next.Next
	return start.Next
}
