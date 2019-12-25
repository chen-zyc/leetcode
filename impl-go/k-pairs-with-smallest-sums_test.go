package leetcode

import (
	"testing"
)

var examples4KSmallestPairs = []struct {
	num1 []int
	num2 []int
	k    int
	want [][]int
}{
	{
		[]int{1, 7, 11},
		[]int{2, 4, 6},
		5,
		[][]int{
			[]int{1, 2}, []int{1, 4}, []int{1, 6}, []int{7, 2}, []int{7, 4},
		},
	},
	{
		[]int{1, 1, 2},
		[]int{1, 2, 3},
		2,
		[][]int{
			[]int{1, 1}, []int{1, 1},
		},
	},
	{
		[]int{1, 2},
		[]int{3},
		3,
		[][]int{
			[]int{1, 3}, []int{2, 3},
		},
	},
	{
		[]int{1, 1, 2},
		[]int{1, 2, 3},
		10,
		[][]int{
			[]int{1, 1}, []int{1, 1}, []int{1, 2}, []int{1, 2}, []int{2, 1}, []int{2, 2}, []int{1, 3}, []int{1, 3}, []int{2, 3},
		},
	},
}

func TestKSmallestPairs(t *testing.T) {
	for _, e := range examples4KSmallestPairs {
		p := KSmallestPairs(e.num1, e.num2, e.k)
		failed := false
		if len(p) != len(e.want) {
			failed = true
		} else {
			for i, pairs1 := range p {
				pairs2 := e.want[i]
				if pairs1[0]+pairs1[1] != pairs2[0]+pairs2[1] {
					failed = true
					break
				}
			}
		}
		if failed {
			t.Fatalf("for %v, got %v", e, p)
		}
	}
}

func TestKSmallestPairs2(t *testing.T) {
	for _, e := range examples4KSmallestPairs {
		p := KSmallestPairs2(e.num1, e.num2, e.k)
		failed := false
		if len(p) != len(e.want) {
			failed = true
		} else {
			for i, pairs1 := range p {
				pairs2 := e.want[i]
				if pairs1[0]+pairs1[1] != pairs2[0]+pairs2[1] {
					failed = true
					break
				}
			}
		}
		if failed {
			t.Fatalf("for %v, got %v", e, p)
		}
	}
}

func TestKSmallestPairs3(t *testing.T) {
	for _, e := range examples4KSmallestPairs {
		p := KSmallestPairs3(e.num1, e.num2, e.k)
		failed := false
		if len(p) != len(e.want) {
			failed = true
		} else {
			for i, pairs1 := range p {
				pairs2 := e.want[i]
				if pairs1[0]+pairs1[1] != pairs2[0]+pairs2[1] {
					failed = true
					break
				}
			}
		}
		if failed {
			t.Fatalf("for %v, got %v", e, p)
		}
	}
}

func TestKSmallestPairs4(t *testing.T) {
	for _, e := range examples4KSmallestPairs {
		p := KSmallestPairs4(e.num1, e.num2, e.k)
		failed := false
		if len(p) != len(e.want) {
			failed = true
		} else {
			for i, pairs1 := range p {
				pairs2 := e.want[i]
				if pairs1[0]+pairs1[1] != pairs2[0]+pairs2[1] {
					failed = true
					break
				}
			}
		}
		if failed {
			t.Fatalf("for %v, got %v", e, p)
		}
	}
}

func BenchmarkKSmallestPairs(b *testing.B) {
	for i := 0; i < b.N; i++ {
		KSmallestPairs([]int{1, 1, 2}, []int{1, 2, 3}, 10)
	}
}

func BenchmarkKSmallestPairs2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		KSmallestPairs2([]int{1, 1, 2}, []int{1, 2, 3}, 10)
	}
}

func BenchmarkKSmallestPairs3(b *testing.B) {
	for i := 0; i < b.N; i++ {
		KSmallestPairs3([]int{1, 1, 2}, []int{1, 2, 3}, 10)
	}
}

func BenchmarkKSmallestPairs4(b *testing.B) {
	for i := 0; i < b.N; i++ {
		KSmallestPairs4([]int{1, 1, 2}, []int{1, 2, 3}, 10)
	}
}
