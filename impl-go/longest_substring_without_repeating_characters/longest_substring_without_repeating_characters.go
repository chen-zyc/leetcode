package longest_substring_without_repeating_characters

func lengthOfLongestSubstring(s string) int {
	n := len(s)
	if n <= 1 {
		return n
	}
	longest := 0
	m := make(map[byte]int, len(s))
	j := 0 // 左边界
	for i := 0; i < n; i++ {
		c := s[i]
		if index, exist := m[c]; exist {
			if l := len(m); l > longest {
				longest = l
			}
			// 将[j, index]从m中删除
			for ; j <= index; j++ {
				delete(m, s[j])
			}
		}
		m[c] = i
	}
	if l := len(m); l > longest {
		longest = l
	}
	return longest
}

func lengthOfLongestSubstring_1(s string) int {
	n := len(s)
	if n <= 1 {
		return n
	}
	max := 0
	m := make(map[byte]int, len(s))
	j := 0 // 左边界
	for i := 0; i < n; i++ {
		c := s[i]
		if index, exist := m[c]; exist {
			if index+1 > j { // 边界只向后移动
				j = index + 1
			}
		}
		m[c] = i
		if x := i - j + 1; x > max {
			max = x
		}
	}
	return max
}


