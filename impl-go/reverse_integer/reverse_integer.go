package reverse_integer

func reverse(x int) int {
	result := int32(0)
	for x != 0 {
		tail := int32(x % 10)
		newRes := result*10 + tail
		if (newRes-tail)/10 != result { // 溢出
			return 0
		}
		result = newRes
		x /= 10
	}
	return int(result)
}
