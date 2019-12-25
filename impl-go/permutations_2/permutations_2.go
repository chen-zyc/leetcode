package permutations_2

import "sort"

func permuteUnique(nums []int) [][]int {
	sort.Ints(nums)
	var result [][]int
	recursion(nums, 0, &result)
	return result
}

func recursion(nums []int, start int, result *[][]int) {
	if start >= len(nums) {
		clone := make([]int, len(nums))
		copy(clone, nums)
		*result = append(*result, clone)
		return
	}
	for i := start; i < len(nums); i++ {
		if i != start && nums[i] == nums[start] {
			continue
		}
		nums[i], nums[start] = nums[start], nums[i]
		// 必须得复制吗？
		numsClone := make([]int, len(nums))
		copy(numsClone, nums)
		recursion(numsClone, start+1, result)
	}
}
