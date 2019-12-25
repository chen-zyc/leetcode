package leetcode

import (
	"fmt"
	"math"
)

/*
Your task is to calculate ab mod 1337 where a is a positive integer and b is an extremely large positive integer given in the form of an array.

Example1:

```
a = 2
b = [3]

Result: 8
```

Example2:

```
a = 2
b = [1,0]

Result: 1024
```
*/

// FAILED
func SuperPow(a int, b []int) int {
	if len(b) == 0 {
		return 0
	}
	res := 1
	last := len(b) - 1
	for i, n := range b {
		if n == 0 {
			continue
		}
		m := (n * int(math.Pow10(last-i)))
		res *= int(math.Pow(float64(a), float64(m))) % 1337
		fmt.Printf("#%d: m:%d, res:%d\n", i, m, res)
	}

	return res
}

// https://discuss.leetcode.com/topic/50489/c-clean-and-short-solution

// tip:a^1234567 % k = (a^1234560 % k) * (a^7 % k) % k = (a^123456 % k)^10 % k * (a^7 % k) % k
func SuperPow2(a int, b []int) int {
	if len(b) == 0 {
		return 1
	}
	lastDigit := b[len(b)-1]
	b = b[:len(b)-1]
	return powMod(SuperPow2(a, b), 10, 1337) * powMod(a, lastDigit, 1337) % 1337
}

// a^k mod 1337 where 0 <= k <= 10
func powMod(a, k, base int) int {
	a %= base
	result := 1
	for i := 0; i < k; i++ {
		result = (result * a) % base
	}
	return result
}

// https://discuss.leetcode.com/topic/51328/7ms-java-solution-using-fast-power-algorithm

func SuperPow3(a int, b []int) int {
	if len(b) == 0 {
		return 0
	}
	a %= 1337
	l := len(b)
	ans := 1
	for i := 0; i < l; i++ {
		ans = (pow(ans, 10, 1337) * pow(a, b[i], 1337)) % 1337
	}
	return ans
}

func pow(a, b, base int) int {
	if b == 1 {
		return a
	}
	if b == 0 {
		return 1
	}
	x := pow(a, b/2, base)
	x = (x * x) % base
	if b&1 == 1 {
		x = (x * a) % base
	}
	return x
}
