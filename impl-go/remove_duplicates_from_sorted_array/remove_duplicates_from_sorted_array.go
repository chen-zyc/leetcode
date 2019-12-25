package remove_duplicates_from_sorted_array

func removeDuplicates(nums []int) int {
	n := len(nums)
	if n < 2 {
		return n
	}
	id := 1
	for i := 1; i < n; i++ {
		if cur, pre := nums[i], nums[i-1]; cur != pre {
			nums[id] = cur
			id++
		}
	}
	return id
}
