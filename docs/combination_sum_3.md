# Combination Sum 3

来源：<https://leetcode.com/problems/combination-sum-iii/>

# 问题

Find all possible combinations of **k** numbers that add up to a number **n**, given that only numbers from 1 to 9 can be used and each combination should be a unique set of numbers.

**Example 1:**

Input: k = 3, n = 7

Output:

```
[[1,2,4]]
```

**Example 2:**

Input: k = 3, n = 9

Output:

```
[[1,2,6], [1,3,5], [2,3,4]]
```

找到所有可能的包含k个数字的组合，使其相加等于n，数字只能使用1-9并且每个组合中的数字都是唯一的。

# 代码

```go
func CombinationSum3(k int, n int) [][]int {
	result := [][]int{}
	combinationSum3Help(k, n, 1, nil, &result)
	return result
}

func combinationSum3Help(k, n, start int, com []int, result *[][]int) {
	if n == 0 && k == 0 {
		comCopy := make([]int, len(com))
		copy(comCopy, com)
		*result = append(*result, comCopy)
		return
	}
	if k > 0 {
		for i := start; i <= 9; i++ {
			if n >= i {
				com = append(com, i)
				combinationSum3Help(k-1, n-i, i+1, com, result)
				com = com[:len(com)-1]
				continue
			}
			return
		}
	}
}
```

思路和 [combination_sum.md](doc/combination_sum.md) 及 [combination_sum_2.md](doc/combination_sum_2.md) 基本一致。

