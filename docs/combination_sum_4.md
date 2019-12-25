# Combination Sum 4

来源：<https://leetcode.com/problems/combination-sum-iv/>

# 问题

Given an integer array with all positive numbers and no duplicates, find the number of possible combinations that add up to a positive integer target.

**Example:**

```
nums = [1, 2, 3]
target = 4

The possible combination ways are:
(1, 1, 1, 1)
(1, 1, 2)
(1, 2, 1)
(1, 3)
(2, 1, 1)
(2, 2)
(3, 1)

Note that different sequences are counted as different combinations.

Therefore the output is 7.
```

给定一个全是正数且不重复的数组，输出和为给定整数的组合的个数。

不同顺序的序列被当做不同的组合。

这个问题有意思了

# 代码1

```go
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
```

首先这个代码在运算 `nums=[1,2,3], target=32` 时超时了，结果有181997601个解，但是思路是一样的。

和之前不同，这里只要输出组合的个数就可以了，所以返回值是 `int` 而不再是 `[][]int`。

这里没有先对 `nums` 进行排序，所以当 `target < num` 时没有 `return`，即使 `return` 了也会超时。

# 代码2

```go
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
```

这里依然使用了递归，与代码1不同的是使用 `cache` 缓存了计算结果，`cache[n] = m` 表示当 `target` 为 `n` 时，组合的个数为 `m` 个。

`cache` 初始化时都为 `-1`，不能为0的原因是：假设都为0，那么判断时就得 `if cache[target] != 0`， 如果当 `target` 为5时有0个组合符合要求，此时缓存中的0就无法判断是没有计算过还是有0个组合。

# 代码3

```go
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
```

代码2是自上而下，代码3是自下而上。

这里假设 `target` 为 `1,2,3,…,target`, 然后每个都计算出来。

思路和前面的代码相同，只是不用递归求解，而是直接从 `comb` 中读取就可以了。

