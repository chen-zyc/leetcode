package combination_sum_2

import "sort"

func CombinationSum2(candidates []int, target int) [][]int {
	sort.Ints(candidates)
	result := [][]int{}
	combinationSum2Help(candidates, target, nil, &result)
	return result
}

func combinationSum2Help(candidates []int, target int, com []int, result *[][]int) {
	if target > 0 {
		for i, c := range candidates {
			// 去掉重复的
			// candidates 中可以有重复的，这会递归中选中重复的，这里去重是保证 candidates 中重复的元素不会被过多的使用
			if i > 0 && c == candidates[i-1] {
				continue
			}
			if target >= c {
				com = append(com, c)
				combinationSum2Help(candidates[i+1:], target-c, com, result)
				com = com[:len(com)-1]
				continue
			}
			return
		}
	}
	if target == 0 {
		comCopy := make([]int, len(com))
		copy(comCopy, com)
		*result = append(*result, comCopy)
		return
	}
}
