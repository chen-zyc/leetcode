package three_sum_closest

import "sort"

func threeSumClosest(nums []int, target int) int {
	sort.Ints(nums)
	n := len(nums)
	// 如果个数小于等于3个，返回他们的和
	if n <= 3 {
		sum := 0
		for _, n := range nums {
			sum += n
		}
		return sum
	}

	ans := nums[0] + nums[1] + nums[2]
	for i := 0; i < n-2; i++ {
		lo, hi := i+1, n-1
		for lo < hi {
			sum := nums[i] + nums[lo] + nums[hi]
			// sum距离target更近
			if abs(target-sum) < abs(target-ans) {
				ans = sum
				if ans == target {
					return ans
				}
			}
			if sum > target {
				hi--
			} else {
				lo++
			}
		}
	}
	return ans
}

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}
