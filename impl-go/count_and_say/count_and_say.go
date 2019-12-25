package count_and_say

import (
	"bytes"
	"strconv"
)

func countAndSay(n int) string {
	buf := bytes.Buffer{}
	pre := "1"
	for i := 1; i < n; i++ {
		c := pre[0]
		count := 1
		for j := 1; j < len(pre); j++ {
			if pre[j] == c {
				count++
				continue
			}
			buf.WriteString(strconv.Itoa(count))
			buf.WriteByte(c)
			c = pre[j]
			count = 1
		}
		buf.WriteString(strconv.Itoa(count))
		buf.WriteByte(c)
		pre = buf.String()
		buf.Reset()
	}
	return pre
}
