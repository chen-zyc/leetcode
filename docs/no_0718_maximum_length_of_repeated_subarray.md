- [718. 最长重复子数组](#718-最长重复子数组)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：动态规划](#方法一动态规划)
    - [方法二：滑动窗口](#方法二滑动窗口)
    - [方法三：二分查找 + 哈希](#方法三二分查找--哈希)
    - [算法导论中的求最长公共子序列](#算法导论中的求最长公共子序列)

------------------------------

# 718. 最长重复子数组

## 题目

给两个整数数组 A 和 B ，返回两个数组中公共的、长度最长的子数组的长度。

示例 1:

```
输入:
A: [1,2,3,2,1]
B: [3,2,1,4,7]
输出: 3
解释: 
长度最长的公共子数组是 [3, 2, 1]。
```

说明:

- 1 <= len(A), len(B) <= 1000
- 0 <= `A[i]`, `B[i]` < 100

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/maximum-length-of-repeated-subarray
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

## 题解

这是一个最长公共子序列的问题，《算法导论》上有。不对，算法导论上的可以不连续，这个是要求**连续的**。


> 链接：https://leetcode-cn.com/problems/maximum-length-of-repeated-subarray/solution/zui-chang-zhong-fu-zi-shu-zu-by-leetcode-solution/

**写在前面**

本题要求我们计算两个数组的最长公共子数组。需要注意到数组长度不超过 1000，且**子数组在原数组中连续**。

容易想到一个暴力解法，即枚举数组 A 中的起始位置 i 与数组 B 中的起始位置 j，然后计算 `A[i:]` 与 `B[j:]` 的**最长公共前缀** k。最终答案即为所有的最长公共前缀的最大值。

> 这里借用了 Python 表示数组的方法，`A[i:]` 表示数组 A 中索引 i 到数组末尾的范围对应的子数组。

这个过程的伪代码如下：

```lua
ans = 0
for i in [0 .. A.length - 1]
    for j in [0 .. B.length - 1]
        k = 0
        while (A[i+k] == B[j+k]) do   # and i+k < A.length etc.
            k += 1
        end while
        ans = max(ans, k)
    end for
end for
```

虽然该暴力解法的最坏时间复杂度为 $O(n^3)$，但通过观察该暴力解法，我们可以提出几个时间复杂度更优秀的解法。



### 方法一：动态规划

**思路及算法**

暴力解法的过程中，我们发现最坏情况下对于任意 i 与 j ，`A[i]` 与 `B[j]` 比较了 $\min(i + 1, j + 1)$ 次。这也是导致了该暴力解法时间复杂度过高的根本原因。

不妨设 A 数组为 `[1, 2, 3]`，B 两数组为为 `[1, 2, 4]`，那么在暴力解法中 `A[2]` 与 `B[2]` 被比较了三次。这三次比较分别是我们计算 `A[0:]` 与 `B[0:]` 最长公共前缀、 `A[1:]` 与 `B[1:]` 最长公共前缀以及 `A[2:]` 与 `B[2:]` 最长公共前缀时产生的。

我们希望优化这一过程，使得任意一对 `A[i]` 和 `B[j]` 都只被比较一次。这样我们自然而然想到利用这一次的比较结果。**如果 `A[i] == B[j]`，那么我们知道 `A[i:]` 与 `B[j:]` 的最长公共前缀为 `A[i + 1:]` 与 `B[j + 1:]` 的最长公共前缀的长度加一，否则我们知道 `A[i:]` 与 `B[j:]` 的最长公共前缀为零**。

这样我们就可以提出动态规划的解法：令 `dp[i][j]` 表示 `A[i:]` 和 `B[j:]` 的最长公共前缀，那么答案即为所有 `dp[i][j]` 中的最大值。如果 `A[i] == B[j]`，那么 `dp[i][j] = dp[i + 1][j + 1] + 1`，否则 `dp[i][j] = 0`。

> 这里借用了 Python 表示数组的方法，`A[i:]` 表示数组 A 中索引 i 到数组末尾的范围对应的子数组。

考虑到这里 `dp[i][j]` 的值从 `dp[i + 1][j + 1]` 转移得到，所以我们需要倒过来，首先计算 `dp[len(A) - 1][len(B) - 1]`，最后计算 `dp[0][0]`。

> 也不一定要从后往前算，也可以从前往后算。
> if `A[i] == B[i]`, `dp[i][j] = dp[i-1][j-1]+1`，就是说 `A[i]` 可以算到前面的子数组中（也就是可以当后缀）。

```go
func findLength(A []int, B []int) int {
    n, m := len(A), len(B)
    dp := make([][]int, n + 1)
    for i := 0; i < len(dp); i++ {
        dp[i] = make([]int, m + 1)
    }
    ans := 0
    for i := n - 1; i >= 0; i-- {
        for j := m - 1; j >= 0; j-- {
            if A[i] == B[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1
            } else {
                dp[i][j] = 0
            }
            if ans < dp[i][j] {
                ans = dp[i][j]
            }
        }
    }
    return ans
}
```

**复杂度分析**

- 时间复杂度： $O(N \times M)$。
- 空间复杂度： $O(N \times M)$。

> N 表示数组 A 的长度，M 表示数组 B 的长度。
> 
> 空间复杂度还可以再优化，利用滚动数组可以优化到 $O(\min(N,M))$。



### 方法二：滑动窗口

**思路及算法**

我们注意到之所以两位置会比较多次，是因为重复子数组在两个数组中的位置可能不同。以 `A = [3, 6, 1, 2, 4], B = [7, 1, 2, 9]` 为例，它们的最长重复子数组是 `[1, 2]`，在 A 与 B 中的开始位置不同。

但如果我们知道了开始位置，我们就可以根据它们将 A 和 B 进行「对齐」，即：

```
A = [3, 6, 1, 2, 4]
B =    [7, 1, 2, 9]
           ↑ ↑
```

此时，最长重复子数组在 A 和 B 中的开始位置相同，我们就可以对这两个数组进行一次遍历，得到子数组的长度，伪代码如下：

```lua
ans = 0
len = min(A.length, B.length)
k = 0
for i in [0 .. len - 1] do
    if (A[i] == B[i]) then
        k = k + 1
    else
        k = 0
    end if
    ans = max(ans, k)
end for
```

> 注意这里指定了 `A[i]` 对齐 `B[i]`，在实际代码实现中会通过指定初始位置 addA 与 addB 的方式来对齐，因此表达式会略有差别。

我们可以枚举 A 和 B 所有的对齐方式。对齐的方式有两类：第一类为 A 不变，B 的首元素与 A 中的某个元素对齐；第二类为 B 不变，A 的首元素与 B 中的某个元素对齐。对于每一种对齐方式，我们计算它们相对位置相同的重复子数组即可。

**为什么两种对齐方式都要检查一遍？**

![](assets/no_0718_maximum_length_of_repeated_subarray3.png)

如果只固定 A，那么是需要这样移动 B 才能遍历所有的情况的。

![](assets/no_0718_maximum_length_of_repeated_subarray1.gif)

```go
func findLength(A []int, B []int) int {
    n, m := len(A), len(B)
    ret := 0
    // 移动 A，让 A[i] 和 B[0] 对齐。
    for i := 0; i < n; i++ {
        len := min(m, n - i)
        maxLen := maxLength(A, B, i, 0, len)
        ret = max(ret, maxLen)
    }
    // 移动 B，让 B[i] 和 A[0] 对齐。
    for i := 0; i < m; i++ {
        len := min(n, m - i)
        maxLen := maxLength(A, B, 0, i, len)
        ret = max(ret, maxLen)        
    }
    return ret
}

func maxLength(A, B []int, addA, addB, len int) int {
    ret, k := 0, 0
    for i := 0; i < len; i++ {
        if A[addA + i] == B[addB + i] {
            k++
        } else {
            k = 0
        }
        ret = max(ret, k)
    }
    return ret
}

func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}

func min(x, y int) int {
    if x < y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度： $O((N + M) \times \min(N, M))$
- 空间复杂度： $O(1)$。

> N 表示数组 A 的长度，M 表示数组 B 的长度。



### 方法三：二分查找 + 哈希

**思路及算法**

如果数组 A 和 B 有一个长度为 k 的公共子数组，那么它们一定有长度为 j <= k 的公共子数组。这样我们可以通过二分查找的方法找到最大的 k。

而为了优化时间复杂度，在二分查找的每一步中，我们可以考虑使用哈希的方法来判断数组 A 和 B 中**是否存在相同特定长度的子数组**。

注意到序列内元素值小于 100 ，我们使用 Rabin-Karp 算法来对序列进行哈希。具体地，我们制定一个素数 base，那么序列 S 的哈希值为：

$$
\mathrm{hash}(S) = \sum_{i=0}^{|S|-1} base^{|S|-(i+1)} \times S[i]
$$

形象地说，就是把 S 看成一个类似 base 进制的数（左侧为高位，右侧为低位），它的十进制值就是这个它的哈希值。由于这个值一般会非常大，因此会将它对另一个素数 mod 取模。

当我们要在一个序列 S​ 中算出所有长度为 len 的子序列的哈希值时，我们可以用类似滑动窗口的方法，在线性时间内得到这些子序列的哈希值。例如，如果我们当前得到了 `S[0:len]` 的哈希值，希望算出 `S[1:len+1]` 的哈希值时，有下面这个公式：

$$
\mathrm{hash}(S[1:len+1]) = (\mathrm{hash}(S[0:len]) - base^{len-1} \times S[0]) \times base + S[len + 1]
$$

> 即删去最高位 `S[0]`，其余位自动进一，并补上最低位 `S[len+1]`。

在二分查找的每一步中，我们使用哈希表分别存储这两个数组的所有长度为 len 的子数组的哈希值，将它们的哈希值进行比对，如果两序列存在相同的哈希值，则认为两序列存在相同的子数组。为了防止哈希碰撞，我们也可以在发现两个子数组的哈希值相等时，进一步校验它们本身是否确实相同，以确保正确性。但该方法在本题中很难发生哈希碰撞，因此略去进一步校验的部分。

```go
const (
    base, mod = 113, 1000000009
)

func findLength(A []int, B []int) int {
    check := func(length int) bool {
        hashA := 0
        for i := 0; i < length; i++ {
            hashA = (hashA * base + A[i]) % mod
        }
        bucketA := map[int]bool{hashA: true}
        mult := qPow(base, length - 1)
        for i := length; i < len(A); i++ {
            hashA = ((hashA - A[i - length] * mult % mod + mod) % mod * base + A[i]) % mod
            bucketA[hashA] = true
        }

        hashB := 0
        for i := 0; i < length; i++ {
            hashB = (hashB * base + B[i]) % mod
        }
        if bucketA[hashB] {
            return true
        }
        for i := length; i < len(B); i++ {
            hashB = ((hashB - B[i - length] * mult % mod + mod) % mod * base + B[i]) % mod
            if bucketA[hashB] {
                return true
            }
        }
        return false
    }

    left, right := 1, min(len(A), len(B)) + 1
    for left < right {
        mid := (left + right) >> 1
        if check(mid) {
            left = mid + 1
        } else {
            right = mid
        }
    }
    return left - 1
}
    
func qPow(x, n int) int {
    ret := 1
    for n != 0 {
        if n & 1 != 0 {
            ret = ret * x % mod
        }
        x = x * x % mod
        n >>= 1
    }
    return ret
}

func min(x, y int) int {
    if x < y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度：$O\big((M+N) \log{(\min(M, N))}\big)$
- 空间复杂度：$O(N)$。

> N 表示数组 A 的长度，M 表示数组 B 的长度。二分查找为对数时间复杂度，计算哈希值的时间复杂度为 $O(M+N)$，哈希检测的时间复杂度为 $O(1)$。



### 算法导论中的求最长公共子序列

![](assets/no_0718_maximum_length_of_repeated_subarray1.png)

![](assets/no_0718_maximum_length_of_repeated_subarray2.png)

需要变动下。相等的时候都是前面的加一，不相等的时候就不要去找左边或上边的最大值了，直接设置为 0.