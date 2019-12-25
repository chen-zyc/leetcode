package generate_parentheses

// 递归的方式

func generateParenthesis(n int) []string {
	var res []string
	addingPar(&res, "", n, 0)
	return res
}

func addingPar(res *[]string, s string, open, close int) {
	if open == 0 && close == 0 {
		*res = append(*res, s)
		return
	}
	if close > 0 {
		addingPar(res, s+")", open, close-1)
	}
	if open > 0 {
		addingPar(res, s+"(", open-1, close+1)
	}
}

// 非递归的方式

func generateParenthesis_1(n int) []string {
	matrix := make([][]string, n+1)
	matrix[0] = []string{""}
	for i := 1; i <= n; i++ {
		list := []string{}
		for j := 0; j < i; j++ { // i种形式
			for _, first := range matrix[j] {
				for _, second := range matrix[i-1-j] {
					list = append(list, "("+first+")"+second)
				}
			}
		}
		matrix[i] = list
	}
	return matrix[n]
}
