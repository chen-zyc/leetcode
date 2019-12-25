package string_to_integer

import "math"

func myAtoi(str string) int {
	// 空串
	if str == "" {
		return 0
	}
	sign, base, i := 1, 0, 0
	// 去除空格
	for str[i] == ' ' {
		i++
	}
	// 符号
	if s := str[i]; s == '-' {
		sign = -1
		i++
	} else if s == '+' {
		i++
	}

	const MAX = math.MaxInt32 / 10
	for n := len(str); i < n; i++ {
		x := int(str[i] - '0')
		if x < 0 || x > 9 {
			break
		}
		if (base > MAX) || (base == MAX && x > 7) {
			if sign == -1 {
				return int(math.MinInt32)
			}
			return int(math.MaxInt32)
		}
		base = 10*base + x
	}
	return base * sign
}
