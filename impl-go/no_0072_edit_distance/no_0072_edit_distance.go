package no_0072_edit_distance

func minDistance(word1 string, word2 string) int {
	m, n := len(word1), len(word2)
	// m 和 n 中有一个为 0
	if m*n == 0 {
		return m + n
	}

	d := make([][]int, m+1)
	for i := 0; i < m+1; i++ {
		d[i] = make([]int, n+1)
	}
	// 初始化 [0][j] 和 [i][0]
	for j := 0; j < n+1; j++ {
		d[0][j] = j
	}
	for i := 0; i < m+1; i++ {
		d[i][0] = i
	}

	for i := 1; i < m+1; i++ {
		for j := 1; j < n+1; j++ {
			left := d[i-1][j] + 1
			up := d[i][j-1] + 1
			leftUp := d[i-1][j-1]
			// d[i][j] 对应的字符串应该是 s[i-1], s[j-1]，因为 0 在 d 中被当作长度，但在 s 中是第一个
			if word1[i-1] != word2[j-1] {
				leftUp++
			}
			d[i][j] = min(left, up, leftUp)
		}
	}
	return d[m][n]
}

func min(a, b, c int) int {
	m := a
	if b < m {
		m = b
	}
	if c < m {
		m = c
	}
	return m
}
