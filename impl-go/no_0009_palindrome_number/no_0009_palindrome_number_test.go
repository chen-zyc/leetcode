package no_0009_palindrome_number

import "testing"

func Test_isPalindrome(t *testing.T) {
	tests := []struct {
		name string
		x    int
		want bool
	}{
		{name: "是回文", x: 121, want: true},
		{name: "负数不是回文", x: -121, want: false},
		{name: "不是回文", x: 10, want: false},
		{name: "0 是回文", x: 0, want: true},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := isPalindrome(tt.x); got != tt.want {
				t.Errorf("isPalindrome() = %v, want %v", got, tt.want)
			}
		})
	}
}
