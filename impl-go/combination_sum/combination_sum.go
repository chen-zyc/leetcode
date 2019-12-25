package combination_sum

import "sort"

// CombinationSum 从candidates中选出和等于target的排列，candidates中的数字可以重复使用
func CombinationSum1(candidates []int, target int) [][]int {
	var solutions [][]int
	if len(candidates) == 0 {
		return solutions
	}

	// 先看第一个元素
	// case 1: 第一个元素正好符合，加到结果里返回
	// case 2: 第一个元素作为被选者，然后再选择剩下的元素
	first := candidates[0]
	if first == target { // 第一个元素就是一种答案
		solutions = append(solutions, []int{first})
	} else if first < target {
		// 选中第一个元素，从candidates中再选择和等于target-first的组合
		// 选出的组合和first共同组成一个解
		subSolutions := CombinationSum1(candidates, target-first)
		for _, subSol := range subSolutions {
			arr := make([]int, len(subSol)+1)
			arr[0] = first
			copy(arr[1:], subSol)
			solutions = append(solutions, arr)
		}
	}

	// 上面都是在选择第一个元素的情况下，下面是将第一个元素排除之后的解
	subSolutions := CombinationSum1(candidates[1:], target)
	if len(subSolutions) > 0 {
		solutions = append(solutions, subSolutions...)
	}
	return solutions
}

// 推荐
// 参考https://discuss.leetcode.com/topic/7698/java-solution-using-recursive改写
func CombinationSum2(candidates []int, target int) [][]int {
	sort.Ints(candidates)
	result := [][]int{}
	combinationSum2Help(nil, candidates, target, &result)
	return result
}

func combinationSum2Help(com []int, candidates []int, target int, result *[][]int) {
	if target == 0 {
		// 复制一份，不然执行 com = com[:len(com)-1] 时会覆盖com的元素
		comCopy := make([]int, len(com))
		copy(comCopy, com)
		*result = append(*result, comCopy)
		return
	}
	if target > 0 {
		// 拿取第一个放到结果里，然后递归再从当前位置开始拿，直到 target 为0说明找到了一个序列
		for i := 0; i < len(candidates); i++ {
			num := candidates[i]
			if target >= num {
				com = append(com, num)
				// 这里从 candidates[i:] 开始查找而不是 i+1， 因为 candidates 中的元素可以重复利用
				combinationSum2Help(com, candidates[i:], target-num, result)
				com = com[:len(com)-1]
				continue
			}
			return
		}
	}

	return
}
