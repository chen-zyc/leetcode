# maximum_subarray

Find the contiguous subarray within an array (containing at least one number) which has the largest sum.

For example, given the array `[-2,1,-3,4,-1,2,1,-5,4]`,
the contiguous subarray `[4,-1,2,1]` has the largest sum = `6`.



```go
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
```

