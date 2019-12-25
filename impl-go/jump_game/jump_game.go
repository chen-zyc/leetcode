package jump_game

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
