package palindrome_number

func isPalindrome(x int) bool {
	// 小于0的，或者最后一位是0的不是回文
	if x < 0 || (x != 0 && x%10 == 0) {
		return false
	}
	rev := 0
	for x > rev { // half of x
		rev = rev*10 + x%10
		x /= 10
	}
	// x => x前半部分，rev => x后半部分的反转
	// x == rev/10的情况：121, 此时x=1，rev=12
	return (x == rev) || (x == rev/10)
}
