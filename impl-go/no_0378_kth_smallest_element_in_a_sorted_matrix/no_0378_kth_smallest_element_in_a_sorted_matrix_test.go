package no_0378_kth_smallest_element_in_a_sorted_matrix

import "testing"

func Test_kthSmallest(t *testing.T) {
	type args struct {
		matrix [][]int
		k      int
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{
			name: "example1",
			args: args{
				matrix: [][]int{
					{1, 5, 9},
					{10, 11, 13},
					{12, 13, 15},
				},
				k: 8,
			},
			want: 13,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := kthSmallest(tt.args.matrix, tt.args.k); got != tt.want {
				t.Errorf("kthSmallest() = %v, want %v", got, tt.want)
			}
		})
	}
}
