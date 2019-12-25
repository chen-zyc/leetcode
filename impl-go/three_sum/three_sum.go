package three_sum

import "sort"

func threeSum(nums []int) [][]int {
	sort.Ints(nums)
	var res [][]int
	for i := 0; i < len(nums)-2; i++ { // 第一个元素
		if i == 0 || (i > 0 && nums[i] != nums[i-1]) { // 不能和之前的重复
			lo, hi, sum := i+1, len(nums)-1, 0-nums[i] // 从两头开始扫描
			for lo < hi {
				if lo != i+1 && nums[lo] == nums[lo-1] {
					lo++
				} else if hi != len(nums)-1 && nums[hi] == nums[hi+1] {
					hi--
				} else if nums[lo]+nums[hi] == sum {
					res = append(res, []int{nums[i], nums[lo], nums[hi]})
					for lo < hi && nums[lo] == nums[lo+1] {
						lo++
					}
					for lo < hi && nums[hi] == nums[hi-1] {
						hi--
					}
					lo++
					hi--
				} else if nums[lo]+nums[hi] < sum {
					// 不足的话去掉小的
					lo++
				} else { // 多的话去掉大的
					hi--
				}
			}
		}
	}
	return res
}
