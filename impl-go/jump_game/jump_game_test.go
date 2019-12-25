package jump_game

import "testing"

func TestCanJump(t *testing.T) {
	var cases = []struct {
		nums []int
		want bool
	}{}
	for _, c := range cases {
		got := canJump(c.nums)
		if got != c.want {
			t.Fatal(got, c)
		}
	}
}
