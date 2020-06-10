package no_0009_palindrome_number

func isPalindrome(x int) bool {
	if x < 0 || (x != 0 && x%10 == 0) {
		return false
	}

	n := 0
	for x > n {
		n = n*10 + x%10 // 把 x 的最后一位追加到 n 的后面
		x /= 10
	}

	return n == x || n/10 == x
}
