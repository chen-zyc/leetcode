package divide_two_integers

import "math"

func divide(dividend int, divisor int) int {
	if divisor == 0 || (dividend == math.MinInt32 && divisor == -1) {
		return math.MaxInt32
	}
	sign := 1
	// 只有一个为负数
	if x1, x2 := dividend < 0, divisor < 0; (x1 && (!x2)) || (x2 && (!x1)) {
		sign = -1
	}
	if dividend < 0 {
		dividend = -dividend
	}
	if divisor < 0 {
		divisor = -divisor
	}
	res := 0
	for dividend >= divisor {
		temp, multiple := divisor, 1
		for dividend >= (temp << 1) {
			temp <<= 1
			multiple <<= 1
		}
		dividend -= temp
		res += multiple
	}
	if sign == -1 {
		return -res
	}
	return res
}
