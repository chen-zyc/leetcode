package no_0070_climbing_stairs

func climbStairs(n int) int {
	// 就一级阶梯，所以只有一种方法
	if n == 1 {
		return 1
	}

	first := 1  // (0, 1)
	second := 2 // (0, 2)
	for i := 3; i <= n; i++ {
		sum := first + second
		first = second
		second = sum
	}
	return second
}
