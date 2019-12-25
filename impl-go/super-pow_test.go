package leetcode

import "testing"

var examples4SuperPow = []struct {
	a      int
	b      []int
	result int
}{
	{2, []int{3}, 8},
	{2, []int{1, 0}, 1024},
	{2147483647, []int{2, 0, 0}, 1198},
}

func _TestSuperPow(t *testing.T) {
	for _, e := range examples4SuperPow {
		p := SuperPow(e.a, e.b)
		if p != e.result {
			t.Errorf("for %v, got %d", e, p)
		}
	}
}

func _BenchmarkSuperPow(b *testing.B) {
	for i := 0; i < b.N; i++ {
		SuperPow(9, []int{1, 0, 2, 4})
	}
}

func TestSuperPow2(t *testing.T) {
	for _, e := range examples4SuperPow {
		p := SuperPow2(e.a, e.b)
		if p != e.result {
			t.Errorf("for %v, got %d", e, p)
		}
	}
}

func BenchmarkSuperPow2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		SuperPow2(9, []int{1, 0, 2, 4})
	}
}

func TestSuperPow3(t *testing.T) {
	for _, e := range examples4SuperPow {
		p := SuperPow3(e.a, e.b)
		if p != e.result {
			t.Errorf("for %v, got %d", e, p)
		}
	}
}

func BenchmarkSuperPow3(b *testing.B) {
	for i := 0; i < b.N; i++ {
		SuperPow3(9, []int{1, 0, 2, 4})
	}
}
