package no_0026_remove_duplicates_from_sorted_array

func removeDuplicates(nums []int) int {
	n := len(nums)
	if n < 2 {
		return n
	}
	i := 0
	for j := 1; j < n; j++ {
		if cur, pre := nums[j], nums[i]; cur != pre {
			i++
			if i != j { // 避免没有必要的赋值
				nums[i] = cur
			}
		}
	}
	return i + 1 // 返回长度
}
