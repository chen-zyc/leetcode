package two_sum

// Given an array of integers, return indices of the two numbers such that they add up to a specific target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// Example:
// ```
// Given nums = [2, 7, 11, 15], target = 9,
// Because nums[0] + nums[1] = 2 + 7 = 9,
// return [0, 1].
// ```

func twoSum(nums []int, target int) []int {
	m := make(map[int]int, len(nums))
	for i, n := range nums {
		m[n] = i // 如果有重复的，记录最后一个的索引
	}
	for i, n := range nums {
		x := target - n
		if y, ok := m[x]; ok && y != i {
			return []int{i, y}
		}
	}
	return nil
}

func twoSum_2(nums []int, target int) []int {
	m := make(map[int]int, len(nums))
	for i, n := range nums {
		x := target - n
		// 查看之前有没有匹配的
		if y, ok := m[x]; ok && y != i {
			return []int{y, i}
		}
		m[n] = i
	}
	return nil
}
