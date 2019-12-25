package longest_palindromic_substring

// 最长回文

func longestPalindrome(s string) string {
	length := len(s)
	if length < 2 {
		return s
	}
	max, left, right := 0, 0, 0
	for i := 0; i < length-1; i++ {
		m, l, r := extendPalindrome(s, i, i)
		if m > max {
			max, left, right = m, l, r
		}
		m, l, r = extendPalindrome(s, i, i+1)
		if m > max {
			max, left, right = m, l, r
		}
	}
	return s[left : right+1]
}

func extendPalindrome(s string, i, j int) (max, left, right int) {
	n := len(s)
	left, right = i, j
	for left >= 0 && right < n && s[left] == s[right] {
		left--
		right++
	}
	left++
	right--
	max = right - left + 1
	return
}
