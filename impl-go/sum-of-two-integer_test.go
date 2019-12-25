package leetcode

import "testing"

func TestGetSum(t *testing.T) {
	examples := []struct {
		a, b int
		want int
	}{
		{1, 2, 3},
		{-1, 2, 1},
		{2, 0, 2},
	}
	for _, e := range examples {
		if sum := GetSum(e.a, e.b); sum != e.want {
			t.Errorf("for %v, got %d", e, sum)
		}
	}
}

func TestGetSubstract(t *testing.T) {
	examples := []struct {
		a, b int
		want int
	}{
		{1, 2, -1},
		{-1, 2, -3},
		{2, 0, 2},
		{199, 80, 119},
	}
	for _, e := range examples {
		if sum := GetSubstract(e.a, e.b); sum != e.want {
			t.Errorf("for %v, got %d", e, sum)
		}
	}
}
