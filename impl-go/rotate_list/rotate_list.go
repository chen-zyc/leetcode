package rotate_list

type ListNode struct {
	Val  int
	Next *ListNode
}

func rotateRight(head *ListNode, k int) *ListNode {
	if head == nil {
		return nil
	}
	// 节点总数
	tail := head
	length := 1
	for tail.Next != nil {
		tail = tail.Next
		length++
	}
	// 首尾相连成环
	tail.Next = head
	k %= length
	if k > 0 {
		for i := 0; i < length-k; i++ {
			tail = tail.Next
		}
	}
	// 现在tail就是最后一个节点
	newHead := tail.Next
	tail.Next = nil
	return newHead
}
