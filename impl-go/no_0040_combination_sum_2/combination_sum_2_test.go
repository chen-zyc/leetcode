package combination_sum_2

import (
	"testing"

	"github.com/zhangyuchen0411/leetcode/utils"
)

func testCombinationSum2(t *testing.T, f func([]int, int) [][]int) {
	examples := []struct {
		input  []int
		target int
		want   [][]int
	}{
		{
			[]int{2, 3, 6, 7},
			7,
			[][]int{
				{7},
			},
		},
		{
			[]int{10, 1, 2, 7, 6, 1, 5},
			8,
			[][]int{
				{1, 7},
				{1, 2, 5},
				{2, 6},
				{1, 1, 6},
			},
		},
	}

	for _, exam := range examples {
		act := f(exam.input, exam.target)
		if !utils.TwoDimenArrEquals(exam.want, act) {
			t.Fatalf("input:%v, target:%d, got:%v, want:%v", exam.input, exam.target, act, exam.want)
		}
	}
}

func TestCombinationSum2(t *testing.T) {
	testCombinationSum2(t, CombinationSum2)
}

func benchCombinationSum2(b *testing.B, f func([]int, int) [][]int) {
	for i := 0; i < b.N; i++ {
		f([]int{10, 1, 2, 7, 6, 1, 5}, 8)
	}
}

func BenchmarkCombinationSum2(b *testing.B) {
	benchCombinationSum2(b, CombinationSum2)
}
