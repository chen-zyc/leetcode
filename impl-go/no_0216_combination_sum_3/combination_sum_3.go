package combination_sum_3

// CombinationSum3 从1-9中选择k个数字使其和等于n
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
