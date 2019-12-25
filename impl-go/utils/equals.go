package utils

import (
	"sort"
)

// TwoDimenArrEquals 比较arr1和arr2是否相等，无关顺序
func TwoDimenArrEquals(arr1, arr2 [][]int) bool {
	if len(arr1) != len(arr2) {
		return false
	}
	if len(arr1) == 0 {
		return true
	}
	for _, ele1 := range arr1 {
		index := -1
		for i, ele2 := range arr2 {
			if ArrEquals(ele1, ele2) {
				index = i
				break
			}
		}
		if index == -1 {
			return false
		}
		arr2 = append(arr2[:index], arr2[index+1:]...)
	}
	return true
}

// ArrEquals 比较arr1和arr2是否相同，返回true当arr1和arr2都为空或者两者元素都相同，即使顺序不同.
// NOTE ArrEquals 会改变数组元素的顺序.
func ArrEquals(arr1, arr2 []int) bool {
	if len(arr1) != len(arr2) {
		return false
	}
	if len(arr1) == 0 {
		return true
	}
	sort.Ints(arr1)
	sort.Ints(arr2)
	for i, ele := range arr1 {
		if ele != arr2[i] {
			return false
		}
	}
	return true
}
