package search_for_a_range

func searchRange(nums []int, target int) []int {
	n := len(nums)
	ret := []int{-1, -1}
	if n < 1 {
		return ret
	}
	i, j := 0, n-1
	// 找到左边的那个
	for i < j {
		mid := (i + j) / 2
		if nums[mid] < target {
			i = mid + 1
		} else {
			j = mid
		}
	}
	if nums[i] != target {
		return ret
	}
	ret[0] = i

	// 找右边的那个
	j = n - 1 // 不重置i
	for i < j {
		mid := (i + j + 1) / 2 // 尽量让mid靠右
		if nums[mid] > target {
			j = mid - 1
		} else {
			i = mid
		}
	}
	ret[1] = j
	return ret
}
