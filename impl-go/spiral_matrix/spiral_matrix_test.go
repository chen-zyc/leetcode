package spiral_matrix

import (
	"reflect"
	"testing"
)

func TestSpiralOrder(t *testing.T) {
	var cases = []struct {
		matrix [][]int
		want   []int
	}{
		{[][]int{
			{1, 2, 3},
			{4, 5, 6},
			{7, 8, 9},
		}, []int{1, 2, 3, 6, 9, 8, 7, 4, 5}},
		{[][]int{
			{2, 3},
		}, []int{2, 3}},
	}

	for _, c := range cases {
		got := spiralOrder(c.matrix)
		if !reflect.DeepEqual(got, c.want) {
			t.Fatal(got, c)
		}
	}
}
