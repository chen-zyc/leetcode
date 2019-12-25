package permutation_sequence

func getPermutation(n int, k int) string {
	// 阶乘 [0!, 1!, 2!, 3!... n!]
	factorial := make([]int, n+1)
	sum := 1
	factorial[0] = 1
	for i := 1; i <= n; i++ {
		sum *= i
		factorial[i] = sum
	}

	// 数字列表 [1, 2, 3, ... n]
	numbers := make([]int, n)
	for i := 0; i < n; i++ {
		numbers[i] = i + 1
	}

	k-- // 从0开始

	res := make([]byte, n)
	for i := 1; i <= n; i++ {
		index := k / factorial[n-i]
		res[i-1] = byte('0' + numbers[index])                   // n在1-9之间
		numbers = append(numbers[:index], numbers[index+1:]...) // 删除被选中的数字
		k -= index * factorial[n-i]
	}
	return string(res)
}
