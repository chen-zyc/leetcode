package longest_substring_without_repeating_characters

import "testing"

func TestLengthOfLongestSubstring(t *testing.T) {
	cases := []struct {
		s    string
		want int
	}{
		{"abcabcbb", 3},
		{"bbbbb", 1},
		{"pwwkew", 3},
		{"aab", 2},
		{"dvdf", 3},
	}

	for _, c := range cases {
		got := lengthOfLongestSubstring(c.s)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}

func BenchmarkLengthOfLongestSubstring(b *testing.B) {
	for i := 0; i < b.N; i++ {
		lengthOfLongestSubstring("abcabcbb")
	}
}

func TestLengthOfLongestSubstring_1(t *testing.T) {
	cases := []struct {
		s    string
		want int
	}{
		{"abcabcbb", 3},
		{"bbbbb", 1},
		{"pwwkew", 3},
		{"aab", 2},
		{"dvdf", 3},
	}

	for _, c := range cases {
		got := lengthOfLongestSubstring_1(c.s)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}

func BenchmarkLengthOfLongestSubstring_1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		lengthOfLongestSubstring_1("abcabcbb")
	}
}
