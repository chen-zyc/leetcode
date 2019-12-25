package search_in_rotated_sorted_array

func search(nums []int, target int) int {
	n := len(nums)
	if n < 1 {
		return -1
	}
	// 1. 使用二分查找找到旋转点
	// 如果中间的比最后的还大，说明旋转点就在中间和最后之间，否则在开始和中间之间
	lo, hi, mid := 0, n-1, 0
	for lo < hi {
		mid = (lo + hi) / 2
		if nums[mid] > nums[hi] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}

	// 现在lo == hi并且是旋转点
	rot := lo
	lo, hi = 0, n-1
	for lo <= hi {
		mid = (lo + hi) / 2
		realMid := (mid + rot) % n
		if nums[realMid] == target {
			return realMid
		}
		if nums[realMid] < target {
			lo = mid + 1
		} else {
			hi = mid - 1
		}
	}
	return -1
}
