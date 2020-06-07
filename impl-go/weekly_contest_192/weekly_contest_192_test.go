package weekly_contest_192

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_shuffle(t *testing.T) {
	res := shuffle([]int{2, 5, 1, 3, 4, 7}, 3)
	require.EqualValues(t, []int{2, 3, 5, 4, 1, 7}, res)

	res = shuffle([]int{1, 2, 3, 4, 4, 3, 2, 1}, 4)
	require.EqualValues(t, []int{1, 4, 2, 3, 3, 2, 4, 1}, res)

	res = shuffle([]int{1, 1, 2, 2}, 2)
	require.EqualValues(t, []int{1, 2, 1, 2}, res)
}

func Test_getStrongest(t *testing.T) {
	equal := func(arr1, arr2 []int) {
		sort.Ints(arr1)
		sort.Ints(arr2)
		require.EqualValues(t, arr1, arr2)
	}
	res := getStrongest([]int{1, 2, 3, 4, 5}, 2)
	equal([]int{5, 1}, res)

	res = getStrongest([]int{1, 1, 3, 5, 5}, 2)
	equal([]int{5, 5}, res)

	res = getStrongest([]int{6, 7, 11, 7, 6, 8}, 5)
	equal([]int{11, 8, 6, 6, 7}, res)

	res = getStrongest([]int{6, -3, 7, 2, 11}, 3)
	equal([]int{-3, 11, 2}, res)

	res = getStrongest([]int{-7, 22, 17, 3}, 2)
	equal([]int{22, 17}, res)
}

func TestBrowserHistory(t *testing.T) {
	browserHistory := Constructor("leetcode.com")
	browserHistory.Visit("google.com")   // 你原本在浏览 "leetcode.com" 。访问 "google.com"
	browserHistory.Visit("facebook.com") // 你原本在浏览 "google.com" 。访问 "facebook.com"
	browserHistory.Visit("youtube.com")  // 你原本在浏览 "facebook.com" 。访问 "youtube.com"
	u := browserHistory.Back(1)          // 你原本在浏览 "youtube.com" ，后退到 "facebook.com" 并返回 "facebook.com"
	require.EqualValues(t, "facebook.com", u)
	u = browserHistory.Back(1) // 你原本在浏览 "facebook.com" ，后退到 "google.com" 并返回 "google.com"
	require.EqualValues(t, "google.com", u)
	u = browserHistory.Forward(1) // 你原本在浏览 "google.com" ，前进到 "facebook.com" 并返回 "facebook.com"
	require.EqualValues(t, "facebook.com", u)
	browserHistory.Visit("linkedin.com") // 你原本在浏览 "facebook.com" 。 访问 "linkedin.com"
	u = browserHistory.Forward(2)        // 你原本在浏览 "linkedin.com" ，你无法前进任何步数。
	require.EqualValues(t, "linkedin.com", u)
	u = browserHistory.Back(2) // 你原本在浏览 "linkedin.com" ，后退两步依次先到 "facebook.com" ，然后到 "google.com" ，并返回 "google.com"
	require.EqualValues(t, "google.com", u)
	u = browserHistory.Back(7) // 你原本在浏览 "google.com"， 你只能后退一步到 "leetcode.com" ，并返回 "leetcode.com"
	require.EqualValues(t, "leetcode.com", u)
}
