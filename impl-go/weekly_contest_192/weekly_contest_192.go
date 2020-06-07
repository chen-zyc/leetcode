package weekly_contest_192

import (
	"container/heap"
	"container/list"
	"sort"
)

// 5428. 重新排列数组
func shuffle(nums []int, n int) []int {
	res := make([]int, 0, len(nums))
	for i, j := 0, n; i < n; i, j = i+1, j+1 {
		res = append(res, nums[i], nums[j])
	}
	return res
}

// 5429. 数组中的 k 个最强值
func getStrongest(arr []int, k int) []int {
	if len(arr) <= k {
		return arr
	}

	sort.Ints(arr)
	mid := arr[(len(arr)-1)/2]

	heapArr := make([]int, k+1)
	copy(heapArr, arr[:k])
	h := &IntHeap{arr: heapArr[:k], mid: mid}
	heap.Init(h)
	for i := k; i < len(arr); i++ {
		heap.Push(h, arr[i])
		heap.Pop(h)
	}

	return h.arr[:k]
}

// // 第 n 小的数
// func getMin(arr []int, n int) int {
//
// }

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

type IntHeap struct {
	arr []int
	mid int
}

func (h *IntHeap) Len() int { return len(h.arr) }

func (h *IntHeap) Less(i, j int) bool {
	m := abs(h.arr[i] - h.mid)
	n := abs(h.arr[j] - h.mid)
	if m > n {
		return false
	}
	if m == n && h.arr[i] > h.arr[j] {
		return false
	}
	return true
}
func (h *IntHeap) Swap(i, j int) {
	h.arr[i], h.arr[j] = h.arr[j], h.arr[i]
}

func (h *IntHeap) Push(x interface{}) {
	h.arr = append(h.arr, x.(int))
}

func (h *IntHeap) Pop() interface{} {
	old := h.arr
	n := len(old)
	x := old[n-1]
	h.arr = old[0 : n-1]
	return x
}

// 5430. 设计浏览器历史记录
type BrowserHistory struct {
	l   *list.List
	cur *list.Element
}

func Constructor(homepage string) BrowserHistory {
	l := list.New()
	cur := l.PushBack(homepage)
	return BrowserHistory{
		l:   l,
		cur: cur,
	}
}

func (this *BrowserHistory) Visit(url string) {
	// 把后面的都删除
	for next := this.cur.Next(); next != nil; next = this.cur.Next() {
		this.l.Remove(next)
	}
	this.cur = this.l.PushBack(url)
}

func (this *BrowserHistory) Back(steps int) string {
	for i := 0; i < steps; i++ {
		if ele := this.cur.Prev(); ele != nil {
			this.cur = ele
		} else {
			break
		}
	}
	return this.cur.Value.(string)
}

func (this *BrowserHistory) Forward(steps int) string {
	for i := 0; i < steps; i++ {
		if ele := this.cur.Next(); ele != nil {
			this.cur = ele
		} else {
			break
		}
	}
	return this.cur.Value.(string)
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * obj := Constructor(homepage);
 * obj.Visit(url);
 * param_2 := obj.Back(steps);
 * param_3 := obj.Forward(steps);
 */
