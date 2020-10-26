- [1365. 有多少小于当前数字的数字](#1365-有多少小于当前数字的数字)
  - [官方题解](#官方题解)
    - [方法一：暴力](#方法一暴力)
    - [方法二：快速排序](#方法二快速排序)
    - [方法三：计数排序](#方法三计数排序)

------------------------------

# 1365. 有多少小于当前数字的数字

给你一个数组 nums，对于其中每个元素 `nums[i]`，请你统计数组中比它小的所有数字的数目。

换而言之，对于每个 `nums[i]` 你必须计算出有效的 j 的数量，其中 j 满足 `j != i` 且 `nums[j] < nums[i]`。

以数组形式返回答案。

示例 1：

```
输入：nums = [8,1,2,2,3]
输出：[4,0,1,1,3]
解释： 
对于 nums[0]=8 存在四个比它小的数字：（1，2，2 和 3）。 
对于 nums[1]=1 不存在比它小的数字。
对于 nums[2]=2 存在一个比它小的数字：（1）。 
对于 nums[3]=2 存在一个比它小的数字：（1）。 
对于 nums[4]=3 存在三个比它小的数字：（1，2 和 2）。
```

示例 2：

```
输入：nums = [6,5,4,8]
输出：[2,1,0,3]
```

示例 3：

```
输入：nums = [7,7,7,7]
输出：[0,0,0,0]
```
 
提示：

- 2 <= nums.length <= 500
- 0 <= `nums[i]` <= 100

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/how-many-numbers-are-smaller-than-the-current-number
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

## 官方题解

> 链接：https://leetcode-cn.com/problems/how-many-numbers-are-smaller-than-the-current-number/solution/you-duo-shao-xiao-yu-dang-qian-shu-zi-de-shu-zi--2/

### 方法一：暴力

比较容易想到的一个方法是，对于数组中的每一个元素，我们都遍历数组一次，统计小于当前元素的数的数目。

```go
func smallerNumbersThanCurrent(nums []int) (ans []int) {
    for _, v := range nums {
        cnt := 0
        for _, w := range nums {
            if w < v {
                cnt++
            }
        }
        ans = append(ans, cnt)
    }
    return
}
```

- 时间复杂度：$O(N^2)$，其中 N 为数组的长度。
- 空间复杂度：$O(1)$。注意我们不计算答案数组的空间占用。

### 方法二：快速排序

我们也可以将数组排序，并记录每一个数在原数组中的位置。对于排序后的数组中的每一个数，我们找出其左侧第一个小于它的数，这样就能够知道数组中小于该数的数量。

```go
type pair struct{ v, pos int }

func smallerNumbersThanCurrent(nums []int) []int {
    n := len(nums)
    data := make([]pair, n)
    for i, v := range nums {
        data[i] = pair{v, i}
    }
    sort.Slice(data, func(i, j int) bool { return data[i].v < data[j].v })
    ans := make([]int, n)
    prev := -1
    for i, d := range data {
        if prev == -1 || d.v != data[i-1].v {
            prev = i
        }
        ans[d.pos] = prev
    }
    return ans
}
```

- 时间复杂度：$O(N\log N)$，其中 N 为数组的长度。排序需要 $O(N\log N)$ 的时间，随后需要 $O(N)$ 时间来遍历。
- 空间复杂度：$O(N)$。因为要额外开辟一个数组。

### 方法三：计数排序

注意到数组元素的值域为 $[0,100]$，所以可以考虑建立一个频次数组 `cnt`，`cnt[i]` 表示数字 i 出现的次数。那么对于数字 i 而言，小于它的数目就为 `cnt[0...i-1]` 的总和。

```go
func smallerNumbersThanCurrent(nums []int) []int {
    cnt := [101]int{}
    for _, v := range nums {
        cnt[v]++
    }
    for i := 0; i < 100; i++ {
        cnt[i+1] += cnt[i]
    }
    ans := make([]int, len(nums))
    for i, v := range nums {
        if v > 0 {
            ans[i] = cnt[v-1]
        }
    }
    return ans
}
```

- 时间复杂度：$O(N + K)$，其中 K 为值域大小。需要遍历两次原数组，同时遍历一次频次数组 cnt 找出前缀和。
- 空间复杂度：$O(K)$。因为要额外开辟一个值域大小的数组。
