package zigzag_conversion

func convert(s string, numRows int) string {
	buf := make([][]byte, numRows)

	length := len(s)
	for i := 0; i < length; {
		// 垂直向下
		for idx := 0; idx < numRows && i < length; idx++ {
			buf[idx] = append(buf[idx], s[i])
			i++
		}
		// 倾斜向上，不包括第一行和最后一行
		for idx := numRows - 2; idx >= 1 && i < length; idx-- {
			buf[idx] = append(buf[idx], s[i])
			i++
		}
	}
	// 全部整合到第一行
	res := buf[0]
	for idx := 1; idx < len(buf); idx++ {
		res = append(res, buf[idx]...)
	}
	return string(res)
}
