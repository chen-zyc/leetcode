# rotate_list

Given a list, rotate the list to the right by *k* places, where *k* is non-negative.

For example:
Given `1->2->3->4->5->NULL` and *k* = `2`,
return `4->5->1->2->3->NULL`.



```go
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
```

