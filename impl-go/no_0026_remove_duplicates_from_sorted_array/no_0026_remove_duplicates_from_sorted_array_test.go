package no_0026_remove_duplicates_from_sorted_array

import (
	"testing"
)

func Test_removeDuplicates(t *testing.T) {
	tests := []struct {
		name string
		nums []int
		want int
	}{
		{
			name: "1,1,2",
			nums: []int{1, 1, 2},
			want: 2,
		},
		{
			name: "0,0,1,1,1,2,2,3,3,4",
			nums: []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4},
			want: 5,
		},
		{
			name: "没有重复项",
			nums: []int{1, 2, 3},
			want: 3,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := removeDuplicates(tt.nums); got != tt.want {
				t.Errorf("removeDuplicates() = %v, want %v", got, tt.want)
			}
		})
	}
}
