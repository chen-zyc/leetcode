package combination_sum_4

import (
	"testing"
)

func testCombinationSum4(t *testing.T, f func([]int, int) int) {
	examples := []struct {
		nums   []int
		target int
		want   int
	}{
		{
			[]int{1, 2, 3}, 4, 7,
		},
		{
			[]int{3, 1, 2, 4}, 4, 8,
		},
		{
			[]int{1, 2, 3}, 32, 181997601,
		},
	}

	for _, exam := range examples {
		act := f(exam.nums, exam.target)
		if act != exam.want {
			t.Fatalf("nums:%v, target:%d, got:%d, want:%d", exam.nums, exam.target, act, exam.want)
		}
	}
}

func TestCombinationSum4(t *testing.T) {
	testCombinationSum4(t, CombinationSum4)
}

func TestCombinationSum4_2(t *testing.T) {
	testCombinationSum4(t, CombinationSum4_2)
}

func TestCombinationSum4_3(t *testing.T) {
	testCombinationSum4(t, CombinationSum4_3)
}

func benchmarkCombinationSum4(b *testing.B, f func([]int, int) int) {
	for i := 0; i < b.N; i++ {
		f([]int{1, 2, 3}, 32)
	}
}

func BenchmarkCombinationSum4(b *testing.B) {
	benchmarkCombinationSum4(b, CombinationSum4)
}

func BenchmarkCombinationSum4_2(b *testing.B) {
	benchmarkCombinationSum4(b, CombinationSum4_2)
}

func BenchmarkCombinationSum4_3(b *testing.B) {
	benchmarkCombinationSum4(b, CombinationSum4_3)
}
