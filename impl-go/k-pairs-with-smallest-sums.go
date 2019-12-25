package leetcode

import (
	"container/heap"
	"math"
	"sort"
)

/*
You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.

Define a pair (u,v) which consists of one element from the first array and one element from the second array.

Find the k pairs (u1,v1),(u2,v2) ...(uk,vk) with the smallest sums.

Example 1:

```
Given nums1 = [1,7,11], nums2 = [2,4,6],  k = 3

Return: [1,2],[1,4],[1,6]

The first 3 pairs are returned from the sequence:
[1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
```

Example 2:

```
Given nums1 = [1,1,2], nums2 = [1,2,3],  k = 2

Return: [1,1],[1,1]

The first 2 pairs are returned from the sequence:
[1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
```

Example 3:

```
Given nums1 = [1,2], nums2 = [3],  k = 3

Return: [1,3],[2,3]

All possible pairs are returned from the sequence:
[1,3],[2,3]
```
*/

type Sum struct {
	n1, n2 int
	sum    int
}

type Sums []*Sum

func (s Sums) Len() int { return len(s) }

func (s Sums) Less(i, j int) bool { return s[i].sum < s[j].sum }

func (s Sums) Swap(i, j int) { s[i], s[j] = s[j], s[i] }

func KSmallestPairs(nums1 []int, nums2 []int, k int) [][]int {
	len1, len2 := len(nums1), len(nums2)
	if len1 == 0 || len2 == 0 || k <= 0 {
		return nil
	}

	sums := make(Sums, 0, len1*len2)
	for _, n1 := range nums1 {
		for _, n2 := range nums2 {
			sums = append(sums, &Sum{
				n1:  n1,
				n2:  n2,
				sum: n1 + n2,
			})
		}
	}

	sort.Sort(sums)

	if k > len(sums) {
		k = len(sums)
	}
	res := make([][]int, k)
	for i := 0; i < k; i++ {
		s := sums[i]
		res[i] = []int{s.n1, s.n2}
	}

	return res
}

// https://discuss.leetcode.com/topic/50527/java-10ms-solution-no-priority-queue

func KSmallestPairs2(nums1 []int, nums2 []int, k int) [][]int {
	len1, len2 := len(nums1), len(nums2)
	if len1 == 0 || len2 == 0 || k <= 0 {
		return nil
	}
	if m := len1 * len2; k > m {
		k = m
	}

	// index类似于突起，向后滑动索引
	index := make([]int, len1)
	ret := make([][]int, 0, k)
	for ; k > 0; k-- {
		minVal := int(math.MaxInt32)
		in := -1
		for i, n := range nums1 {
			ind := index[i]
			if ind >= len2 {
				continue
			}
			if s := n + nums2[ind]; s < minVal {
				minVal = s
				in = i
			}
		}
		if in == -1 {
			break
		}
		ret = append(ret, []int{nums1[in], nums2[index[in]]})
		index[in]++
	}
	return ret
}

type Item struct {
	value    []int // The value of the item; arbitrary.
	priority int   // The priority of the item in the queue.
	// The index is needed by update and is maintained by the heap.Interface methods.
	index int // The index of the item in the heap.
}

// A PriorityQueue implements heap.Interface and holds Items.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int { return len(pq) }

func (pq PriorityQueue) Less(i, j int) bool {
	// We want Pop to give us the highest, not lowest, priority so we use greater than here.
	return pq[i].priority > pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	item.index = -1 // for safety
	*pq = old[0 : n-1]
	return item
}

func KSmallestPairs3(nums1 []int, nums2 []int, k int) [][]int {
	len1, len2 := len(nums1), len(nums2)
	if len1 == 0 || len2 == 0 || k <= 0 {
		return nil
	}
	if m := len1 * len2; k > m {
		k = m
	}

	ret := make([][]int, 0, k)

	queue := make(PriorityQueue, 0, k)

	heap.Push(&queue, &Item{
		value:    []int{0, 0},
		priority: nums1[0] + nums2[0],
	})

	for k > 0 && queue.Len() > 0 {
		item := heap.Pop(&queue).(*Item)
		i, j := item.value[0], item.value[1]
		//fmt.Println("pop\t", "i:", i, "\tj:", j)
		ret = append(ret, []int{nums1[i], nums2[j]})
		k--

		if j+1 < len2 {
			heap.Push(&queue, &Item{
				value:    []int{i, j + 1},
				priority: -(nums1[i] + nums2[j+1]),
			})
			//fmt.Println("push\t", "i:", i, "\tj:", j+1, "\tprio:", nums1[i]+nums2[j+1])
		}
		if j == 0 && i+1 < len1 {
			heap.Push(&queue, &Item{
				value:    []int{i + 1, 0},
				priority: -(nums1[i+1] + nums2[0]),
			})
			//fmt.Println("push\t", "i:", i+1, "\tj:", 0, "\tprio:", nums1[i+1]+nums2[0])
		}
	}
	return ret
}

// https://discuss.leetcode.com/topic/50908/heap-java-o-k-log-min-n-k-8ms-solution-with-briefly-explanation

func KSmallestPairs4(nums1 []int, nums2 []int, k int) [][]int {
	len1, len2 := len(nums1), len(nums2)
	if len1 == 0 || len2 == 0 || k <= 0 {
		return nil
	}
	if m := len1 * len2; k > m {
		k = m
	}

	ret := make([][]int, 0, k)

	m := int(math.Min(float64(k), float64(len1)))
	queue := make(PriorityQueue, m)

	// nums1的所有元素和nums2[0]组合
	for i := 0; i < m; i++ {
		queue[i] = &Item{
			value:    []int{i, 0},
			priority: -(nums1[i] + nums2[0]),
			index:    i,
		}
	}
	heap.Init(&queue)

	for i := 0; i < k; i++ {
		item := heap.Pop(&queue).(*Item)
		row, col := item.value[0], item.value[1]
		//fmt.Println("pop\t", "row:", row, "\tcol:", col)
		if col < len2-1 {
			heap.Push(&queue, &Item{
				value:    []int{row, col + 1},
				priority: -(nums1[row] + nums2[col+1]),
			})
			//fmt.Println("push\t", "row:", row, "\tcol:", col+1, "\tprio:", nums1[row]+nums2[col+1])
		}
		ret = append(ret, []int{nums1[row], nums2[col]})
	}

	return ret
}
