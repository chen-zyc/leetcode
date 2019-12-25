package leetcode

import "math"

var numPick int

func guess(n int) int {
	if numPick > n {
		return 1
	} else if numPick < n {
		return -1
	} else {
		return 0
	}
}

func guessNumber(n int) int {
	i := 1
	j := n
	for i < j {
		// NOTE 这里不能用(i+j)/2，因为i+j可能会溢出，一旦溢出就会陷入无限循环
		mid := i + (j-i)/2
		if re := guess(mid); re == 0 {
			return mid
		} else if re == 1 {
			i = mid + 1
		} else {
			j = mid
		}
	}
	return -1
}

/*
We are playing the Guess Game. The game is as follows:

I pick a number from 1 to n. You have to guess which number I picked.

Every time you guess wrong, I'll tell you whether the number I picked is higher or lower.

However, when you guess a particular number x, and you guess wrong, you pay $x. You win the game when you guess the number I picked.

Example:

n = 10, I pick 8.

First round:  You guess 5, I tell you that it's higher. You pay $5.
Second round: You guess 7, I tell you that it's higher. You pay $7.
Third round:  You guess 9, I tell you that it's lower. You pay $9.

Game over. 8 is the number I picked.

You end up paying $5 + $7 + $9 = $21.
Given a particular n ≥ 1, find out how much money you need to have to guarantee a win.
*/

// https://discuss.leetcode.com/topic/51353/simple-dp-solution-with-explanation

func GetMoneyAmount1(n int) int {
	table := make([][]int, n+1)
	for i := range table {
		table[i] = make([]int, n+1)
	}
	return dp(table, 1, n)
}

func dp(t [][]int, s, e int) int {
	if s >= e {
		return 0
	}
	if c := t[s][e]; c != 0 {
		return c
	}
	res := int(math.MaxInt32)
	for i := s; i <= e; i++ {
		// 如果选择的是i，money表示最多需要花费多少。
		money := i + max(dp(t, s, i-1), dp(t, i+1, e))
		res = min(res, money)
	}
	t[s][e] = res
	return res
}

func max(n1, n2 int) int {
	if n1 > n2 {
		return n1
	}
	return n2
}

func min(n1, n2 int) int {
	if n1 < n2 {
		return n1
	}
	return n2
}

func GetMoneyAmount2(n int) int {
	table := make([][]int, n+1)
	for i := range table {
		table[i] = make([]int, n+1)
	}

	for j := 2; j <= n; j++ {
		for i := j - 1; i > 0; i-- {
			globalMin := int(math.MaxInt32)
			for k := i + 1; k < j; k++ {
				localMax := k + max(table[i][k-1], table[k+1][j])
				globalMin = min(globalMin, localMax)
			}
			if i+1 == j {
				table[i][j] = i
			} else {
				table[i][j] = globalMin
			}
		}
	}

	return table[1][n]
}
