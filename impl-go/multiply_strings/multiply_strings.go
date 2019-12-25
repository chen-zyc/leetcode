package multiply_strings

import (
	"bytes"
	"strconv"
)

func multiply(num1 string, num2 string) string {
	n1, n2 := len(num1), len(num2)
	n := n1 + n2
	result := make([]int, n)

	// 一定要从后向前
	for i := n1 - 1; i >= 0; i-- {
		for j := n2 - 1; j >= 0; j-- {
			mul := int((num1[i] - '0') * (num2[j] - '0'))
			mul += result[i+j+1]
			result[i+j] += mul / 10
			// 这里是等于号，因为是从后向前遍历，这个值在这一步已经被确定了
			result[i+j+1] = mul % 10
		}
	}

	// result转换成字符串

	// 去掉0
	i := 0
	for ; i < n && result[i] == 0; i++ {
	}
	if i == n {
		return "0"
	}

	buf := bytes.NewBuffer(make([]byte, 0, n-i))
	for ; i < n; i++ {
		buf.WriteString(strconv.Itoa(result[i])) // 这里result[i]不一定只有一位
	}
	return buf.String()
}
