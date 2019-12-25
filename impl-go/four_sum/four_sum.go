package four_sum

import (
	"container/list"
	"sort"
)

func fourSum(nums []int, target int) [][]int {
	length := len(nums)
	if length < 4 {
		return [][]int{}
	}
	res := list.New()
	sort.Ints(nums)
	// 4个最小的都比target大 || 4个最大的还没有target大，其他组合都不会满足target
	max := nums[length-1]
	if 4*nums[0] > target || 4*max < target {
		return [][]int{}
	}

	for i := 0; i < length; i++ {
		z := nums[i]
		// 避免重复
		if i > 0 && z == nums[i-1] {
			continue
		}
		// z 太小了
		if z+3*max < target {
			continue
		}
		// z 太大了, 而且nums后面的都比z大
		if 4*z > target {
			break
		}
		// z 是边界值
		if 4*z == target {
			// z后面3个都等于z
			if i+3 < length && nums[i+3] == z {
				res.PushBack([]int{z, z, z, z})
			}
			break
		}
		threeSumForFourSum(nums, target-z, i+1, length-1, res, z)
	}
	arr := make([][]int, 0, res.Len())
	for e := res.Front(); e != nil; e = e.Next() {
		arr = append(arr, e.Value.([]int))
	}
	return arr
}

func threeSumForFourSum(nums []int, target, low, high int, res *list.List, z1 int) {
	if low+1 >= high { // high - low <= 1
		return
	}

	max := nums[high]
	if 3*nums[low] > target || 3*max < target {
		return
	}

	for i := low; i < high-1; i++ {
		z := nums[i]
		if i > low && nums[i-1] == z { // 避免重复
			continue
		}
		if z+2*max < target { // z太小了
			continue
		}
		if 3*z > target { // z太大了
			break
		}
		if 3*z == target { // 边界值
			if i+1 < high && nums[i+2] == z {
				res.PushBack([]int{z1, z, z, z})
			}
			break
		}
		twoSumForFourSum(nums, target-z, i+1, high, z1, z, res)
	}
}

func twoSumForFourSum(nums []int, target, low, high, z1, z2 int, res *list.List) {
	if low >= high {
		return
	}
	if 2*nums[low] > target || 2*nums[high] < target {
		return
	}
	for low < high {
		sum := nums[low] + nums[high]
		if sum == target {
			x1 := nums[low]
			x2 := nums[high]
			res.PushBack([]int{z1, z2, x1, x2})
			// 避免重复
			for low++; low < high && x1 == nums[low]; low++ {
			}
			for high--; low < high && x2 == nums[high]; high-- {
			}
		} else if sum < target {
			low++
		} else {
			high--
		}
	}
}
