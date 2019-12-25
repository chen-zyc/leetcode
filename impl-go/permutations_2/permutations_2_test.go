package permutations_2

import "testing"

func TestPermuteUnique(t *testing.T) {
	res := permuteUnique([]int{2, 2, 1, 1})
	if len(res) != 6 {
		t.Fatal(res)
	}
}
