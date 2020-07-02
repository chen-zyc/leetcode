package no_0378_kth_smallest_element_in_a_sorted_matrix

import (
	"container/heap"
	"fmt"
)

func kthSmallest(matrix [][]int, k int) int {
	h := &IHeap{}
	// 把每行第一个元素放到最小堆里
	for i := 0; i < len(matrix); i++ {
		fmt.Println("放进：", matrix[i][0])
		heap.Push(h, [3]int{matrix[i][0], i, 0})
	}

	// 弹出了 k-1 个元素，放进去了 **最多** k-1 个，怎么保证的堆里有 k 个元素？
	// 不需要保证堆里有 k 个元素，这是是把前 k-1 个最小的元素都弹出去了，再弹一次就是第 k 小的元素了。
	for i := 0; i < k-1; i++ {
		now := heap.Pop(h).([3]int) // 弹出最小的那个
		fmt.Println("弹出：", now[0])
		if now[2] != len(matrix)-1 {
			// 这一行下一列的数
			heap.Push(h, [3]int{matrix[now[1]][now[2]+1], now[1], now[2] + 1})
			fmt.Println("放进：", matrix[now[1]][now[2]+1])
		}
	}
	return heap.Pop(h).([3]int)[0]
}

type IHeap [][3]int

func (h IHeap) Len() int           { return len(h) }
func (h IHeap) Less(i, j int) bool { return h[i][0] < h[j][0] }
func (h IHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IHeap) Push(x interface{}) {
	*h = append(*h, x.([3]int))
}

func (h *IHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}
