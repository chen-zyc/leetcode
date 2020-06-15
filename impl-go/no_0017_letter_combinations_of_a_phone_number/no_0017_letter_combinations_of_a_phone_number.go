package letter_combinations_of_a_phone_number

func letterCombinations(digits string) []string {
	if digits == "" {
		return []string{}
	}
	ans := []string{""}
	mapping := []string{"0", "1", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"}
	for _, d := range digits {
		chars := mapping[d-'0']
		temp := make([]string, 0, len(chars)*len(ans))
		for _, s := range chars {
			for _, a := range ans {
				temp = append(temp, a+string(s))
			}
		}
		ans = temp
	}
	return ans
}
