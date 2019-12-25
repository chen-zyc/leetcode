package medianoftwosortedarrays

import "math"

func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	m, n := len(nums1), len(nums2)
	if m > n { // to ensure m <= n
		nums1, nums2 = nums2, nums1
		m, n = n, m
	}
	iMin, iMax := 0, m
	halfLen := (m + n + 1) / 2
	for iMin <= iMax {
		i := (iMin + iMax) / 2
		j := halfLen - i
		if i < iMax && nums2[j-1] > nums1[i] {
			iMin = i + 1 // i is too small
			continue
		}
		if i > iMin && nums1[i-1] > nums2[j] {
			iMax = i - 1 // i is too big
			continue
		}
		// i is perfect
		maxLeft := float64(0)
		if i == 0 {
			maxLeft = float64(nums2[j-1])
		} else if j == 0 {
			maxLeft = float64(nums1[i-1])
		} else {
			maxLeft = math.Max(float64(nums1[i-1]), float64(nums2[j-1]))
		}
		if (m+n)%2 == 1 {
			return maxLeft
		}

		minRight := float64(0)
		if i == m {
			minRight = float64(nums2[j])
		} else if j == n {
			minRight = float64(nums1[i])
		} else {
			minRight = math.Min(float64(nums1[i]), float64(nums2[j]))
		}
		return (maxLeft + minRight) / 2
	}
	return 0.0
}
