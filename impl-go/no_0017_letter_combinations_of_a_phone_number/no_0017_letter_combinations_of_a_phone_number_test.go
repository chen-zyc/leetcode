package letter_combinations_of_a_phone_number

import "testing"

func TestLetterCombinations(t *testing.T) {
	cases := []struct {
		d   string
		ans []string
	}{
		{"23", []string{"ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"}},
	}
	for _, c := range cases {
		got := letterCombinations(c.d)
		if !sliceEqual(got, c.ans) {
			t.Fatal(c, got)
		}
	}
}

func sliceEqual(s1, s2 []string) bool {
	m := make(map[string]struct{}, len(s1))
	for _, s := range s1 {
		m[s] = struct{}{}
	}
	for _, s := range s2 {
		if _, ok := m[s]; ok {
			delete(m, s)
		} else {
			return false
		}
	}
	return len(m) == 0
}
