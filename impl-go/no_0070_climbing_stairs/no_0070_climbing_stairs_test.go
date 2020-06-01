package no_0070_climbing_stairs

import (
	"testing"
)

func Test_climbStairs(t *testing.T) {
	tests := []struct {
		name string
		n    int
		want int
	}{
		{name: "1级", n: 1, want: 1},
		{name: "2级", n: 2, want: 2},
		{name: "3级", n: 3, want: 3},
		{name: "4级", n: 4, want: 5},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := climbStairs(tt.n); got != tt.want {
				t.Errorf("climbStairs() = %v, want %v", got, tt.want)
			}
		})
	}
}
