package integer_to_roman

func intToRoman(num int) string {
	var M = []string{"", "M", "MM", "MMM"}
	var C = []string{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"}
	var X = []string{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"}
	var I = []string{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"}
	return M[num/1000] + C[(num%1000)/100] + X[(num%100)/10] + I[num%10]
}
