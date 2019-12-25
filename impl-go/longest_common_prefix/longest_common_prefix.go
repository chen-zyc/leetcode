package longest_common_prefix

func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}
	pre := strs[0]
	strs = strs[1:]
	for i, c := range pre {
		for _, s := range strs {
			if i >= len(s) || s[i] != byte(c) {
				return pre[:i]
			}
		}
	}
	return pre
}
