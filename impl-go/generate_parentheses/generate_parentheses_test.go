package generate_parentheses

import "testing"

func BenchmarkGen(b *testing.B) {
	for i := 0; i < b.N; i++ {
		generateParenthesis(3)
	}
}

func BenchmarkGen1(b *testing.B) {
	for i := 0; i < b.N; i++ {
		generateParenthesis_1(3)
	}
}
