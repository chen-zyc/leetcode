package string_to_integer

import "testing"

func TestMyAtoi(t *testing.T) {
	cases := []struct {
		s    string
		want int
	}{
		{"", 0},
		{"1", 1},
		{"2147483648", 2147483647},
	}
	for _, c := range cases {
		got := myAtoi(c.s)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}
