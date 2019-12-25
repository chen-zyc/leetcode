package Implement_strStr

func strStr(haystack string, needle string) int {
	haystackLen := len(haystack)
	needleLen := len(needle)
	for i := 0; ; i++ { // haystack 起始位置
		for j := 0; ; j++ {
			if j == needleLen {
				return i
			}
			if i+j == haystackLen {
				return -1
			}
			if needle[j] != haystack[i+j] {
				break
			}
		}
	}
}
