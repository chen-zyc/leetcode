package no_0015_three_sum

import "sort"

func threeSum(nums []int) [][]int {
	n := len(nums)
	sort.Ints(nums)
	var res [][]int

	// a + b + c = 0
	for a := 0; a < n; a++ {
		// 过滤掉和上次相同的元素，防止出现重复的组合
		if a != 0 && nums[a] == nums[a-1] {
			continue
		}

		// c 指针从右开始遍历, b 指针从左开始遍历：b -> ... <- c
		c := n - 1
		target := -nums[a] // b + c = -a
		for b := a + 1; b < n; b++ {
			// 过滤掉和上次相同的元素，防止出现重复的组合
			if b != a+1 && nums[b] == nums[b-1] {
				continue
			}
			// 固定 b 指针，左移 c 指针，直到 [b] + [c] <= target.
			// 同时 b 指针要保证在 c 左侧
			for b < c && nums[b]+nums[c] > target {
				c--
			}

			// 现在相等了，那么 b 右移时 c 就会出现在 b 左侧了，这里直接 break。
			// 为什么不放在下面再判断？如果放在下面，会出现 -4+2+2=0 这种组合，但题目的意思应该是 a,b,c 不重复。
			if b == c {
				break
			}
			// 现在：1)b >= c; 2) [b]+[c] <= target.
			if nums[b]+nums[c] == target {
				res = append(res, []int{nums[a], nums[b], nums[c]})
			}
		}
	}
	return res
}
