# jump_game

Given an array of non-negative integers, you are initially positioned at the first index of the array.

Each element in the array represents your maximum jump length at that position.

Determine if you are able to reach the last index.

For example:
A = `[2,3,1,1,4]`, return `true`.

A = `[3,2,1,0,4]`, return `false`.



注意标记的是最大跳跃距离，比如3，可以跳1，2，3下。



```go
func canJump(nums []int) bool {
	// reach表示能够跳到的最远的距离
	i, n := 0, len(nums)
	for reach := 0; i < n && i <= reach; i++ { // i<=reach:只能在最远距离之内活动
		newReach := i + nums[i] // 如果从当前位置开始起跳能跳到的最远距离
		if newReach > reach {
			reach = newReach
		}
	}
	return i == n // i到最后了，说明最远距离超过n了
}
```

