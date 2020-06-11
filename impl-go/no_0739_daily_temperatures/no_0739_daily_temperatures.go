package no_0739_daily_temperatures

import (
	"container/list"
)

func dailyTemperatures(T []int) []int {
	res := make([]int, len(T))
	stack := list.New()
	for i, temperature := range T {
		for ele := stack.Front(); ele != nil; ele = stack.Front() {
			idx := ele.Value.(int)
			// 如果在栈中找到一个比当前温度低的，那么说明比那个温度稍高的就是当前的温度
			if T[idx] < temperature {
				res[idx] = i - idx
				stack.Remove(ele)
			} else {
				// 栈中的元素是单调递减的，如果 T[idx] >= temperature, 那么剩下的也会 >=.
				break
			}
		}
		stack.PushFront(i)
	}
	return res
}
