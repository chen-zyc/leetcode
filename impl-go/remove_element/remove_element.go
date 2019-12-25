package remove_element

func removeElement(nums []int, val int) int {
	n := len(nums)
	for i := 0; i < n; {
		if nums[i] == val {
			// 用最后一个替换，减少交换的次数
			nums[i] = nums[n-1]
			n--
		} else {
			i++
		}
	}
	return n
}
