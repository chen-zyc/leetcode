package two_sum

import (
	"reflect"
	"testing"
)

func TestTwoSum(t *testing.T) {
	cases := []struct {
		nums   []int
		target int
		answer []int
	}{
		{[]int{2, 7, 11, 15}, 9, []int{0, 1}},
		{[]int{3, 2, 4}, 6, []int{1, 2}},
		{[]int{3, 3}, 6, []int{0, 1}},
	}
	for _, c := range cases {
		got := twoSum(c.nums, c.target)
		if !reflect.DeepEqual(got, c.answer) {
			t.Fatal(c, got)
		}
	}
}

func TestTwoSum_2(t *testing.T) {
	cases := []struct {
		nums   []int
		target int
		answer []int
	}{
		{[]int{2, 7, 11, 15}, 9, []int{0, 1}},
		{[]int{3, 2, 4}, 6, []int{1, 2}},
		{[]int{3, 3}, 6, []int{0, 1}},
	}
	for _, c := range cases {
		got := twoSum_2(c.nums, c.target)
		if !reflect.DeepEqual(got, c.answer) {
			t.Fatal(c, got)
		}
	}
}
