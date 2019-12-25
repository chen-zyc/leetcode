package leetcode

import "testing"

func TestGuessNumber(t *testing.T) {
	numPick = 6
	n := guessNumber(10)
	if n != 6 {
		t.Errorf("want 6, got %d", n)
	}
}
