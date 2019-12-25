package next_permutation

func nextPermutation(nums []int) {
	n := len(nums)
	if n < 2 {
		return
	}
	index := n - 1
	// 从后向前遍历，找到第一个前面比后面的小的那个位置
	// 循环之后的效果就是：[index .. n-1] 是逐渐减小的
	for ; index > 0; index-- {
		if nums[index-1] < nums[index] {
			break
		}
	}

	// 整个数组都是倒序的，所以变成正序的
	if index == 0 {
		reverseSort(nums, 0, n-1)
		return
	}

	val := nums[index-1]
	j := n - 1
	// 在 nums[index .. n-1] 中找到比val大的最小的数值和val交换
	for ; j >= index && nums[j] <= val; j-- {
	}
	nums[j], nums[index-1] = val, nums[j]

	// 让nums[index..n-1]倒序
	reverseSort(nums, index, n-1)
}

// 反转nums. 123 => 321
func reverseSort(nums []int, start, end int) {
	if start > end {
		return
	}
	for i := start; i <= (end+start)/2; i++ {
		nums[i], nums[start+end-i] = nums[start+end-i], nums[i]
	}
}
