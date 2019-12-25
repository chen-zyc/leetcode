package combination_sum

import (
	"testing"

	"github.com/zhangyuchen0411/leetcode/utils"
)

func testCombinationSum(t *testing.T, f func([]int, int) [][]int) {
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
				{2, 2, 3},
			},
		},
		{
			[]int{1, 2},
			4,
			[][]int{
				{1, 1, 1, 1},
				{1, 1, 2},
				{2, 2},
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

func TestCombinationSum1(t *testing.T) {
	testCombinationSum(t, CombinationSum1)
}

func TestCombinationSum2(t *testing.T) {
	testCombinationSum(t, CombinationSum2)
}

func benchCombinationSum(b *testing.B, f func([]int, int) [][]int) {
	for i := 0; i < b.N; i++ {
		f([]int{2, 3, 6, 7}, 7)
	}
}

func BenchmarkCombinationSum1(b *testing.B) {
	benchCombinationSum(b, CombinationSum1)
}

func BenchmarkCombinationSum2(b *testing.B) {
	benchCombinationSum(b, CombinationSum2)
}
