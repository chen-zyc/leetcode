- [696. 计数二进制子串](#696-计数二进制子串)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：按字符分组](#方法一按字符分组)

------------------------------

# 696. 计数二进制子串

## 题目

给定一个字符串 s，计算具有相同数量0和1的非空(连续)子字符串的数量，并且这些子字符串中的所有0和所有1都是组合在一起的。

重复出现的子串要计算它们出现的次数。

示例 1 :

```
输入: "00110011"
输出: 6
解释: 有6个子串具有相同数量的连续1和0：“0011”，“01”，“1100”，“10”，“0011” 和 “01”。

请注意，一些重复出现的子串要计算它们出现的次数。

另外，“00110011”不是有效的子串，因为所有的0（和1）没有组合在一起。
```

示例 2 :

```
输入: "10101"
输出: 4
解释: 有4个子串：“10”，“01”，“10”，“01”，它们具有相同数量的连续1和0。
```

注意：

- s.length 在1到50,000之间。
- s 只包含“0”或“1”字符。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/count-binary-substrings
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/count-binary-substrings/solution/ji-shu-er-jin-zhi-zi-chuan-by-leetcode-solution/


### 方法一：按字符分组

**思路与算法**

我们可以将字符串 s 按照 0 和 1 的连续段分组，存在 counts 数组中，例如 `s = 00111011`，可以得到这样的 counts 数组：`counts={2,3,1,2}`。

这里 counts 数组中两个相邻的数一定代表的是两种不同的字符。假设 counts 数组中两个相邻的数字为 u 或者 v，它们对应着 u 个 0 和 v 个 1，或者 u 个 1 和 v 个 0。它们能组成的满足条件的子串数目为 `min{u,v}`，即一对相邻的数字对答案的贡献。

我们只要遍历所有相邻的数对，求它们的贡献总和，即可得到答案。

不难得到这样的实现：

```go
func countBinarySubstrings(s string) int {
    counts := []int{}
    ptr, n := 0, len(s)
    for ptr < n {
        c := s[ptr]
        count := 0
        for ptr < n && s[ptr] == c {
            ptr++
            count++
        }
        counts = append(counts, count)
    }
    ans := 0
    for i := 1; i < len(counts); i++ {
        ans += min(counts[i], counts[i-1])
    }
    return ans
}

func min(x, y int) int {
    if x < y {
        return x
    }
    return y
}
```

这个实现的时间复杂度和空间复杂度都是 $O(n)$。

对于某一个位置 i，其实我们只关心 i - 1 位置的 counts 值是多少，所以可以用一个 last 变量来维护当前位置的前一个位置，这样可以省去一个 counts 数组的空间。

代码如下。

```go
func countBinarySubstrings(s string) int {
    var ptr, last, ans int
    n := len(s)
    for ptr < n {
        c := s[ptr]
        count := 0
        for ptr < n && s[ptr] == c {
            ptr++
            count++
        }
        ans += min(count, last)
        last = count
    }

    return ans
}

func min(x, y int) int {
    if x < y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度：$O(n)$。
- 空间复杂度：$O(1)$。
