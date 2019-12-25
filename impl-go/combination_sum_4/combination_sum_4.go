package combination_sum_4

// 使用递归的方式求 [1, 2, 3], 32时超时
func CombinationSum4(nums []int, target int) int {
	if target == 0 {
		return 1
	}
	c := 0
	if target > 0 {
		for _, num := range nums {
			if target >= num {
				// 从 nums 中选出和为 target-num 的组合数。
				// 这是假设num是选出的组合中的第一个元素
				c += CombinationSum4(nums, target-num)
			}
			// 这里没有对 nums 排序，所以不知道 num 后面的元素是否大于 target，所以不能 break, 性能相对差些
		}
	}
	return c
}

// CombinationSum4_2 使用一个数组暂存计算过的结果
// 效率相对于上面来说有了飞速提升
// 参考 https://discuss.leetcode.com/topic/52302/1ms-java-dp-solution-with-detailed-explanation
func CombinationSum4_2(nums []int, target int) int {
	cache := make([]int, target+1)
	for i := 0; i < len(cache); i++ {
		cache[i] = -1
	}
	// 当target变为0时意味着有一个组合符合条件了，所以返回1,对应于上面的 if target == 0 { return 1 }
	cache[0] = 1
	return combinationSum4_2Help(nums, target, cache)
}

func combinationSum4_2Help(nums []int, target int, cache []int) int {
	// 已经计算过了, 直接返回结果
	// 这个成立是建立在每次递归都对整个 nums 进行扫描的情况下
	if v := cache[target]; v != -1 {
		return v
	}
	res := 0
	for _, num := range nums {
		if target >= num {
			res += combinationSum4_2Help(nums, target-num, cache)
		}
	}
	cache[target] = res // 缓存结果
	return res
}

// 上面是自上而下，这个实现是自下而上
// 参考 https://discuss.leetcode.com/topic/52302/1ms-java-dp-solution-with-detailed-explanation
func CombinationSum4_3(nums []int, target int) int {
	comb := make([]int, target+1)
	comb[0] = 1

	for i := 1; i < len(comb); i++ {
		// 这里的 i 相当于 target, for 的作用是填充 comb
		for _, num := range nums {
			if i >= num {
				// 假设num被选中,comb[i-num]为0说明没有组合匹配，否则就是匹配的组合数
				comb[i] += comb[i-num]
			}
		}
	}

	return comb[target]
}
