package container_with_most_water

import "testing"

func TestMaxArea(t *testing.T) {
	cases := []struct {
		heights []int
		want    int
	}{
		{[]int{1, 1}, 1},
		{[]int{1, 8, 6, 2, 5, 4, 8, 3, 7}, 49},
	}
	for _, c := range cases {
		got := maxArea(c.heights)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}
