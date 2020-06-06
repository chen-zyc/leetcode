package no_0010_regular_expression_matching

func isMatch(s string, p string) bool {
	if len(p) == 0 {
		return len(s) == 0
	}

	firstMatch := len(s) != 0 && (s[0] == p[0] || p[0] == '.')

	if len(p) > 1 && p[1] == '*' {
		return (firstMatch && isMatch(s[1:], p)) || // 消掉 s 中的一个字符
			(isMatch(s, p[2:])) // 把自己消掉
	}
	return firstMatch && isMatch(s[1:], p[1:]) // 按位匹配
}

func isMatch2(s string, p string) bool {
	memo := make([][]int, len(s)+1)
	for i := 0; i <= len(s); i++ {
		memo[i] = make([]int, len(p)+1)
	}
	return dp2(s, p, 0, 0, memo)
}

func dp2(s, p string, i, j int, memo [][]int) bool {
	if memo[i][j] != 0 {
		return memo[i][j] == 1 // 1: true, 2: false, 0: 没有值。
	}

	ans := false
	if j == len(p) { // 有一个到末尾了
		ans = i == len(s)
	} else {
		firstMatch := i < len(s) && (s[i] == p[j] || p[j] == '.')
		// 下面的不管 i+1, j+1, j+2 都可能会是 len(s)|len(p)，所以 memo 要比 len(s)|len(p) 多一个长度。
		if j+1 < len(p) && p[j+1] == '*' { // 下一个是 '*'
			ans = (firstMatch && dp2(s, p, i+1, j, memo)) ||
				dp2(s, p, i, j+2, memo)
		} else {
			ans = firstMatch && dp2(s, p, i+1, j+1, memo)
		}
	}
	if ans {
		memo[i][j] = 1
	} else {
		memo[i][j] = 2
	}
	return ans
}

func isMatch3(s string, p string) bool {
	slen := len(s)
	plen := len(p)
	memo := make([][]int, slen+1)
	for i := 0; i <= len(s); i++ {
		memo[i] = make([]int, plen+1)
	}
	memo[slen][plen] = 1 // 1: true, 2: false

	for i := slen; i >= 0; i-- {
		for j := plen - 1; j >= 0; j-- {
			firstMatch := i < slen && (s[i] == p[j] || p[j] == '.')
			if j+1 < plen && p[j+1] == '*' {
				if (firstMatch && memo[i+1][j] == 1) || memo[i][j+2] == 1 {
					memo[i][j] = 1
				} else {
					memo[i][j] = 2
				}
			} else {
				if firstMatch && memo[i+1][j+1] == 1 {
					memo[i][j] = 1
				} else {
					memo[i][j] = 2
				}
			}
		}
	}
	return memo[0][0] == 1
}
