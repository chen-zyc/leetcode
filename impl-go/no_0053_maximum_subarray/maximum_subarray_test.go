package maximum_subarray

import "testing"

func TestMaxSubArray(t *testing.T) {
	var cases = []struct {
		nums []int
		want int
	}{
		{[]int{-1}, -1},
		{[]int{-2, 1, -3, 4, -1, 2, 1, -5, 4}, 6},
		{[]int{-2, 1}, 1},
		{[]int{-2, -1}, -1},
	}
	for _, c := range cases {
		got := maxSubArray(c.nums)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}
