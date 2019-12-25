package longest_palindromic_substring

import "testing"

func TestLongestPalindrome(t *testing.T) {
	cases := []struct {
		in  string
		out string
	}{
		{"babad", "bab"},
	}
	for _, c := range cases {
		got := longestPalindrome(c.in)
		if got != c.out {
			t.Fatal(c, got)
		}
	}
}
