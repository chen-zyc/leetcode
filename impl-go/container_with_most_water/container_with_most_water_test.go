package container_with_most_water

import "testing"

func TestMaxArea(t *testing.T) {
	cases := []struct {
		heights []int
		want    int
	}{
		{[]int{1, 1}, 1},
	}
	for _, c := range cases {
		got := maxArea(c.heights)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}
