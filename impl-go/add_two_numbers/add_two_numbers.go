package add_two_numbers

// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order and each of their nodes contain a single digit.
// Add the two numbers and return it as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
// Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
// Output: 7 -> 0 -> 8
// 342 + 465 = 807

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	root := &ListNode{Next: &ListNode{}} // 哨兵
	n := root
	sum := 0
	for l1 != nil || l2 != nil {
		if l1 != nil {
			sum += l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			sum += l2.Val
			l2 = l2.Next
		}
		n = n.Next
		n.Val = sum % 10
		n.Next = &ListNode{}
		sum /= 10
	}
	if sum > 0 {
		n.Next.Val = sum
	} else {
		n.Next = nil
	}
	return root.Next
}
