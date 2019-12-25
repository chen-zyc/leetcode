package leetcode

/*
Calculate the sum of two integers a and b, but you are not allowed to use the operator + and -.
*/

// https://discuss.leetcode.com/topic/49771/java-simple-easy-understand-solution-with-explanation/2

func GetSum(a int, b int) int {
	if a == 0 {
		return b
	}
	if b == 0 {
		return a
	}

	for b != 0 {
		// 计算进位，a & b bit=1的位说明这位上都是1，应该进位，<<1就是把进位加到前面一位上
		carry := (a & b) << 1
		a = a ^ b // calculate sum of a and b without thinking the carry
		b = carry
		// 现在a表示没有进位的和，b表示进位，两者相加就是a+b
		// 当b为0表示没有进位了，那么a就是和
	}
	return a
}

func GetSubstract(a, b int) int {
	for b != 0 {
		// 计算借位，a&b能表示都是1的位，而对a取反说明a原来是0，就是 0 - 1这种情况需要借位
		borrow := (^a) & b
		a = a ^ b
		b = borrow << 1
	}
	return a
}
