package multiply_strings

import "testing"

func TestMultiply(t *testing.T) {
	var cases = []struct {
		n1, n2 string
		want   string
	}{
		{"45", "123", "5535"},
		{"123", "45", "5535"},
		{"99", "9", "891"},
		{"12", "0", "0"},
	}

	for _, c := range cases {
		got := multiply(c.n1, c.n2)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}
