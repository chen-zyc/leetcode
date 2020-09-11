package combination_sum_3

import (
	"testing"

	"github.com/zhangyuchen0411/leetcode/utils"
)

func testCombinationSum3(t *testing.T, f func(int, int) [][]int) {
	examples := []struct {
		k    int
		n    int
		want [][]int
	}{
		{
			3,
			7,
			[][]int{
				{1, 2, 4},
			},
		},
		{
			3,
			9,
			[][]int{
				{1, 2, 6},
				{1, 3, 5},
				{2, 3, 4},
			},
		},
	}

	for _, exam := range examples {
		act := f(exam.k, exam.n)
		if !utils.TwoDimenArrEquals(exam.want, act) {
			t.Fatalf("k:%d, n:%d, got:%v, want:%v", exam.k, exam.n, act, exam.want)
		}
	}
}

func TestCombinationSum3(t *testing.T) {
	testCombinationSum3(t, CombinationSum3)
}

func benchCombinationSum3(b *testing.B, f func(int, int) [][]int) {
	for i := 0; i < b.N; i++ {
		f(3, 9)
	}
}

func BenchmarkCombinationSum3(b *testing.B) {
	benchCombinationSum3(b, CombinationSum3)
}
