package maximum_subarray

func maxSubArray(nums []int) int {
	n := len(nums)
	if n == 0 {
		return 0
	}
	sum := nums[0]
	maxSum := sum
	for i := 1; i < n; i++ {
		if sum <= 0 {
			sum = nums[i]
		} else {
			sum += nums[i]
		}
		if sum > maxSum {
			maxSum = sum
		}
	}
	return maxSum
}
