package container_with_most_water

func maxArea(height []int) int {
	water, i, j := 0, 0, len(height)-1
	for i < j {
		// 选择较矮的边计算面积
		h := min(height[i], height[j])
		water = max(water, (j-i)*h)
		// 左边右移，宽度较小，必须选择高度更大的
		for height[i] <= h && i < j {
			i++
		}
		// 右边左移，宽度较小，必须选择高度更大的
		for height[j] <= h && i < j {
			j--
		}
	}
	return water
}

func min(i, j int) int {
	if i < j {
		return i
	}
	return j
}

func max(i, j int) int {
	if i > j {
		return i
	}
	return j
}
