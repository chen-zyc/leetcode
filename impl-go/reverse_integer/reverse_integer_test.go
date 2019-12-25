package reverse_integer

import "testing"

func TestReverse(t *testing.T) {
	cases := []struct {
		x    int
		want int
	}{
		{0, 0},
		{123, 321},
		{1534236469, 0},
	}
	for _, c := range cases {
		got := reverse(c.x)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}
