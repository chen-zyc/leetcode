package count_and_say

import (
	"bytes"
	"testing"
)

func TestCountAndSay(t *testing.T) {
	cases := []struct {
		n    int
		want string
	}{
		{1, "1"},
		{2, "11"},
		{3, "21"},
		{4, "1211"},
		{5, "111221"},
	}
	for _, c := range cases {
		got := countAndSay(c.n)
		if got != c.want {
			t.Fatal(c, got)
		}
	}
}

func TestA(t *testing.T) {
	buf := bytes.Buffer{}
	buf.WriteByte(byte('0' + 1))
	buf.WriteByte('2')
	t.Log(buf.String())
}
