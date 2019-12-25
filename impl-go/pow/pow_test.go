package pow

import (
	"math"
	"testing"
)

func TestMyPow(t *testing.T) {
	var cases = []struct {
		x    float64
		n    int
		want float64
	}{
		{1.0, 0, 1},
		{1.0, 1, 1.0},
		{3.4, 2, 11.56},
		{8.88023, 3, 700.2814829},
		{8.0, 5, 32768},
		{8.84372, 5, 54097.2165304},
		{8.84372, -5, 0.00002},
		{8.95371, -1, 0.11169},
		{4.0, -6, 0.0002441},
		{4.70975, -6, 0.00009},
	}
	for _, c := range cases {
		got := myPow(c.x, c.n)
		if math.Abs(c.want-got) > 0.00001 {
			t.Fatal(c, got)
		}
	}
}
