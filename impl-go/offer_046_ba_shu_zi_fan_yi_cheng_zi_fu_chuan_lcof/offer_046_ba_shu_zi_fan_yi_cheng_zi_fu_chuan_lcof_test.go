package lcof

import "testing"

func Test_translateNum(t *testing.T) {
	tests := []struct {
		name string
		num  int
		want int
	}{
		{name: "case 1", num: 1, want: 1},
		{name: "case 2", num: 10, want: 2},
		{name: "case 3", num: 25, want: 2},
		{name: "case 4", num: 12258, want: 5},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := translateNum(tt.num); got != tt.want {
				t.Errorf("translateNum() = %v, want %v", got, tt.want)
			}
		})
	}
}

func BenchmarkTranslateNum(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = translateNum(12)
	}
}

func BenchmarkTranslateNum2(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = translateNum2(12)
	}
}
