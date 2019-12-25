package permutation_sequence

import "testing"

func TestGetPermutation(t *testing.T) {
	var cases = []struct {
		n, k int
		want string
	}{
		{1, 1, "1"},
		{4, 14, "3142"},
	}
	for _, c := range cases {
		got := getPermutation(c.n, c.k)
		if got != c.want {
			t.Fatal(got, c)
		}
	}
}
