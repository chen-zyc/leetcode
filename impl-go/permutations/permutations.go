package permutations

func permute(nums []int) [][]int {
	var result [][]int
	permuteRecursive(nums, 0, &result)
	return result
}

func permuteRecursive(nums []int, begin int, result *[][]int) {
	if begin >= len(nums) {
		clone := make([]int, len(nums))
		copy(clone, nums)
		*result = append(*result, clone)
		return
	}
	for i := begin; i < len(nums); i++ {
		nums[begin], nums[i] = nums[i], nums[begin]
		permuteRecursive(nums, begin+1, result)
		nums[begin], nums[i] = nums[i], nums[begin]
	}
}
