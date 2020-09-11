package lcof

import (
	"strconv"
)

func translateNum(num int) int {
	if num < 10 { // 只在小于 10 的时候有优势
		return 1
	}

	s := strconv.Itoa(num)         // len >= 2
	prev0, prev1, prev2 := 0, 1, 1 // 空字符串时算作一种方法
	for i := 1; i < len(s); i++ {
		prev0, prev1 = prev1, prev2
		if prev := s[i-1 : i+1]; "10" <= prev && prev <= "25" {
			prev2 = prev1 + prev0
		} else {
			prev2 = prev1
		}
	}
	return prev2
}

func translateNum2(num int) int {
	src := strconv.Itoa(num)
	p, q, r := 0, 0, 1
	for i := 0; i < len(src); i++ {
		p, q, r = q, r, 0
		r += q
		if i == 0 {
			continue
		}
		pre := src[i-1 : i+1]
		if pre <= "25" && pre >= "10" {
			r += p
		}
	}
	return r
}
