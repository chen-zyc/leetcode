- [977. 有序数组的平方](#977-有序数组的平方)
  - [官方题解](#官方题解)
    - [方法一：直接排序](#方法一直接排序)
    - [方法二：双指针](#方法二双指针)
    - [方法三：双指针](#方法三双指针)

------------------------------

# 977. 有序数组的平方

给定一个按非递减顺序排序的整数数组 A，返回每个数字的平方组成的新数组，要求也按非递减顺序排序。

 
示例 1：

```
输入：[-4,-1,0,3,10]
输出：[0,1,9,16,100]
```

示例 2：

```
输入：[-7,-3,2,3,11]
输出：[4,9,9,49,121]
```
 
提示：

- 1 <= A.length <= 10000
- -10000 <= `A[i]` <= 10000
- A 已按非递减顺序排序。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/squares-of-a-sorted-array
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/squares-of-a-sorted-array/solution/you-xu-shu-zu-de-ping-fang-by-leetcode-solution/

### 方法一：直接排序

最简单的方法就是将数组 A 中的数平方后直接排序。

```go
func sortedSquares(a []int) []int {
    ans := make([]int, len(a))
    for i, v := range a {
        ans[i] = v * v
    }
    sort.Ints(ans)
    return ans
}
```

**复杂度分析**

- 时间复杂度：$O(n \log n)$，其中 n 是数组 A 的长度。
- 空间复杂度：$O(\log n)$。除了存储答案的数组以外，我们需要 $O(\log n)$ 的栈空间进行排序。

### 方法二：双指针

方法一没有利用「数组 A 已经按照升序排序」这个条件。显然，如果数组 A 中的所有数都是非负数，那么将每个数平方后，数组仍然保持升序；如果数组 A 中的所有数都是负数，那么将每个数平方后，数组会保持降序。

这样一来，如果我们能够找到数组 A 中负数与非负数的分界线，那么就可以用类似「归并排序」的方法了。具体地，我们设 `neg` 为数组 A 中负数与非负数的分界线，也就是说，`A[0]` 到 `A[neg]` 均为负数，而 `A[neg+1]` 到 `A[n−1]` 均为非负数。当我们将数组 A 中的数平方后，那么 `A[0]` 到 `A[neg]` 单调递减，`A[neg+1]` 到 `A[n−1]` 单调递增。

由于我们得到了两个已经有序的子数组，因此就可以使用归并的方法进行排序了。具体地，使用两个指针分别指向位置 `neg` 和 `neg+1`，每次比较两个指针对应的数，选择较小的那个放入答案并移动指针。当某一指针移至边界时，将另一指针还未遍历到的数依次放入答案。

```go
func sortedSquares(a []int) []int {
    n := len(a)
    lastNegIndex := -1
    for i := 0; i < n && a[i] < 0; i++ {
        lastNegIndex = i
    }

    ans := make([]int, 0, n)
    for i, j := lastNegIndex, lastNegIndex+1; i >= 0 || j < n; {
        if i < 0 {
            // 左边没有了，直接把右边的数放进去
            ans = append(ans, a[j]*a[j])
            j++
        } else if j == n {
            // 右边没有了，直接把左边的数放进去
            ans = append(ans, a[i]*a[i])
            i--
        } else if a[i]*a[i] < a[j]*a[j] {
            // 选择较小的那个放进去
            ans = append(ans, a[i]*a[i])
            i--
        } else {
            ans = append(ans, a[j]*a[j])
            j++
        }
    }

    return ans
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 是数组 A 的长度。
- 空间复杂度：$O(1)$。

### 方法三：双指针

同样地，我们可以使用两个指针分别指向位置 0 和 n-1，每次比较两个指针对应的数，选择较大的那个**逆序**放入答案并移动指针。这种方法无需处理某一指针移动至边界的情况，读者可以仔细思考其精髓所在。

```go
func sortedSquares(a []int) []int {
    n := len(a)
    ans := make([]int, n)
    i, j := 0, n-1
    for pos := n - 1; pos >= 0; pos-- {
        if v, w := a[i]*a[i], a[j]*a[j]; v > w {
            ans[pos] = v
            i++
        } else {
            ans[pos] = w
            j--
        }
    }
    return ans
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 是数组 A 的长度。
- 空间复杂度：$O(1)$。
