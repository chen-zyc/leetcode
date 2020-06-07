package no_0136_single_number

func singleNumber(nums []int) int {
	n := 0
	for _, num := range nums {
		n ^= num
	}
	return n
}
