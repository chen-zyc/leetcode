package add_two_numbers

import (
	"reflect"
	"testing"
)

func TestAddTwoNumbers(t *testing.T) {
	cases := []struct {
		l1  []int
		l2  []int
		out []int
	}{
		{[]int{2, 4, 3}, []int{5, 6, 4}, []int{7, 0, 8}},
		{[]int{0}, []int{0}, []int{0}},
		{[]int{1, 8}, []int{0}, []int{1, 8}},
		{[]int{9}, []int{9}, []int{8, 1}},
	}
	for _, c := range cases {
		got := addTwoNumbers(toNode(c.l1), toNode(c.l2))
		out := fromNode(got)
		if !reflect.DeepEqual(out, c.out) {
			t.Fatal(c, out)
		}
	}
}

func BenchmarkAddTwoNumbers(b *testing.B) {
	l1, l2 := toNode([]int{2, 4, 3}), toNode([]int{5, 6, 4})
	for i := 0; i < b.N; i++ {
		addTwoNumbers(l1, l2)
	}
}

func toNode(in []int) (root *ListNode) {
	list := &ListNode{}
	root = list
	for _, n := range in {
		node := &ListNode{Val: n}
		list.Next = node
		list = node
	}
	return root.Next
}

func fromNode(list *ListNode) []int {
	r := []int{}
	for list != nil {
		r = append(r, list.Val)
		list = list.Next
	}
	return r
}
