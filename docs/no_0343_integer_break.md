- [343. 整数拆分](#343-整数拆分)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：动态规划](#方法一动态规划)
    - [方法二：优化的动态规划](#方法二优化的动态规划)
    - [方法三：数学](#方法三数学)

------------------------------

# 343. 整数拆分

## 题目

给定一个正整数 n，将其拆分为**至少**两个正整数的和，并使这些整数的乘积最大化。 返回你可以获得的最大乘积。

示例 1:

```
输入: 2
输出: 1
解释: 2 = 1 + 1, 1 × 1 = 1。
```

示例 2:

```
输入: 10
输出: 36
解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。
```

说明: 你可以假设 n 不小于 2 且不大于 58。

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/integer-break
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/integer-break/solution/zheng-shu-chai-fen-by-leetcode-solution/

### 方法一：动态规划

对于正整数 $n$，当 $n \ge 2$ 时，可以拆分成至少两个正整数的和。令 $k$ 是拆分出的第一个正整数，则剩下的部分是 $n-k$，$n-k$ 可以不继续拆分，或者继续拆分成至少两个正整数的和。由于**每个正整数对应的最大乘积取决于比它小的正整数对应的最大乘积**，因此可以使用动态规划求解。

创建数组 $\text{dp}$，其中 $\text{dp}[i]$ 表示将正整数 $i$ 拆分成至少两个正整数的和之后，这些正整数的最大乘积。特别地，$0$ 不是正整数，$1$ 是最小的正整数，$0$ 和 $1$ 都不能拆分，因此 $\text{dp}[0]=\text{dp}[1]=0$。

当 $i \ge 2$ 时，假设对正整数 $i$ 拆分出的第一个正整数是 $j$（$1 \le j < i$），则有以下两种方案：

- 将 $i$ 拆分成 $j$ 和 $i-j$ 的和，且 $i-j$ 不再拆分成多个正整数，此时的乘积是 $j \times (i-j)$；
- 将 $i$ 拆分成 $j$ 和 $i-j$ 的和，且 $i-j$ 继续拆分成多个正整数，此时的乘积是 $j \times \text{dp}[i-j]$。

因此，当 $j$ 固定时，有 $\text{dp}[i]=\max(j \times (i-j), j \times \text{dp}[i-j])$。由于 $j$ 的取值范围是 $1$ 到 $i-1$，需要遍历所有的 $j$ 得到 $\text{dp}[i]$ 的最大值，因此可以得到状态转移方程如下：

$$
\text{dp}[i]=\mathop{\max}\limits_{1 \le j < i}\{\max(j \times (i-j), j \times \text{dp}[i-j])\}
$$

最终得到 $\text{dp}[n]$ 的值即为将正整数 $n$ 拆分成至少两个正整数的和之后，这些正整数的最大乘积。

```go
func integerBreak(n int) int {
    dp := make([]int, n + 1)
    for i := 2; i <= n; i++ {
        curMax := 0
        for j := 1; j < i; j++ {
            curMax = max(curMax, max(j * (i - j), j * dp[i - j]))
        }
        dp[i] = curMax
    }
    return dp[n]
}

func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度：$O(n^2)$，其中 n 是给定的正整数。对于从 2 到 n 的每一个整数都要计算对应的 dp 值，计算一个整数对应的 dp 值需要 $O(n)$ 的时间复杂度，因此总时间复杂度是 $O(n^2)$。
- 空间复杂度：$O(n)$，其中 n 是给定的正整数。创建一个数组 dp，其长度为 n+1。


### 方法二：优化的动态规划

方法一中定义的状态转移方程如下：

$$
\text{dp}[i]=\mathop{\max}\limits_{1 \le j < i}\{\max(j \times (i-j), j \times \text{dp}[i-j])\}
$$

使用上述状态转移方程，计算 $\text{dp}[i]$ 时，$j$ 的值遍历了从 $1$ 到 $i-1$ 的所有值，因此总时间复杂度是 $O(n^2)$。是否可以降低时间复杂度？

上述状态转移方程包含两项，当 $j$ 固定时，$\text{dp}[i]$ 的值由 $j \times (i-j)$ 和 $j \times \text{dp}[i-j]$ 中的较大值决定，因此需要对两项分别考虑。

首先考虑 $j \times \text{dp}[i-j]$ 这一项。

注意到 $\text{dp}$ 的定义，$\text{dp}[i]$ 表示将正整数 $i$ 拆分成至少两个正整数的和之后，这些正整数的最大乘积，因此对于任意 $1 \le j < i$，有 $\text{dp}[i] \ge j \times \text{dp}[i-j]$(因为 `dp[i]` 是这些值里面最大的那个)。

当 $j$ 是奇数时，有 $j=\frac{j-1}{2}+\frac{j+1}{2}$，因此 $\text{dp}[i] \geq \frac{j-1}{2} \times \text{dp}[i - \frac{j-1}{2}] \ge \frac{j-1}{2} \times \frac{j+1}{2} \times \text{dp}[i-j]$。

当 $j$ 是偶数时，有 $j=\frac{j}{2}+\frac{j}{2}$，因此 $\text{dp}[i] \ge \frac{j}{2} \times \text{dp}[i - \frac{j}{2}] \ge \frac{j}{2} \times \frac{j}{2} \times \text{dp}[i-j]$。

如果 $j \ge 4$ 且 $j$ 是奇数，则 $\frac{j-1}{2} \times \frac{j+1}{2}>j$ 恒成立。如果 $j \ge 4$ 且 $j$ 是偶数，则 $\frac{j}{2} \times \frac{j}{2} \ge j$ 恒成立，当且仅当 $j=4$ 时等号成立。

由此可知，如果 $j \ge 4$，则 $\text{dp}[j] \ge j$，当且仅当 $j=4$ 时等号成立，即**当 $j \ge 4$ 时一定能将 $j$ 拆成至少两个正整数的和，这些正整数的乘积大于或等于 $j$**。

同时也可以得到，如果 $j \ge 4$，则 $\text{dp}[i] \ge j \times \text{dp}[i-j]$，只有当 $j=4$ 时等号可能成立。又由于

$$
\text{dp}[i] \ge 2 \times \text{dp}[i-2] \ge 2 \times (2 \times \text{dp}[i-4]) = 4 \times \text{dp}[i-4]
$$

因此取 $j=2$ 计算得到的 $\text{dp}[i]$ 一定不会小于取 $j=4$ 计算得到的 $\text{dp}[i]$。根据上述分析，**$j \ge 4$ 的情况都是不需要考虑的**。

那么 $j=1$ 是否需要考虑？答案是不需要。如果取 $j=1$，则有 $\text{dp}[i] \ge 1 \times \text{dp}[i-1]=\text{dp}[i-1]$。当 $i \ge 3$ 时，$\text{dp}[i-1]$ 是将正整数 $i-1$ 拆分成至少两个正整数的和之后，这些正整数的最大乘积，在拆分成的正整数中，任选一个数字加 $1$，则拆分成的正整数的和变成 $i$，且乘积一定大于 $\text{dp}[i-1]$，因此必有 $\text{dp}[i]>\text{dp}[i-1]$，即当 $j=1$ 时不可能得到最大的 $\text{dp}[i]$ 的值。

根据上述分析可知，计算 $\text{dp}[i]$ 的值只需要考虑 $j=2$ 和 $j=3$ 的情况，不需要遍历从 $1$ 到 $i-1$ 的所有值。

其次考虑 $j \times (i-j)$ 这一项。

根据上述推导可知，如果 $j \ge 4$，则 $\text{dp}[j] \ge j$，当且仅当 $j=4$ 时等号成立。因此在 $i-j \ge 4$ 的情况下，有 $\text{dp}[i-j] \ge i-j$, $\text{dp}[i] \ge j \times \text{dp}[i-j] \ge j \times (i-j)$，此时计算 $\text{dp}[i]$ 的值不需要考虑 $j \times (i-j)$ 的值。

如果 $i-j<4$，在计算 $\text{dp}[i]$ 的值的时候就需要考虑 $j \times (i-j)$ 的值。在考虑 $j \times \text{dp}[i-j]$ 时，根据上述分析，只需要考虑 $j=2$ 和 $j=3$ 的情况。在考虑 $j \times (i-j)$ 时，需要考虑 $j$ 的哪些值？

如果 $j=1$，则 $1 \times (i-1)=i-1$，当 $i=2$ 或 $i=3$ 时有 $\text{dp}[i]=i-1$，当 $i \ge 4$ 时有 $\text{dp}[i] \ge i>i-1$，显然当 $i \ge 4$ 时取 $j=1$ 不可能得到最大乘积，因此 $j=1$ 时是不需要考虑的。

如果 $j \ge 4$，$\text{dp}[i]$ 是否可能等于 $j \times (i-j)$？当 $i$ 固定时，要使得 $j \times (i-j)$ 的值最大，$j$ 的值应该取 $j=i/2$，这里的 $/$ 表示整数除法。当 $j \ge 4$ 时，若要满足 $j=i/2$，则 $i \ge 8$，此时 $i-j \ge 4$，利用上述结论，$\text{dp}[i-j] \ge i-j$，因此 $j \times \text{dp}[i-j] \ge j \times (i-j)$。由此可见，当 $j \ge 4$ 时，计算 $\text{dp}[i]$ 只需要考虑 $j \times \text{dp}[i-j]$，不需要考虑 $j \times (i-j)$。

又由于在使用 $j \times \text{dp}[i-j]$ 计算 $\text{dp}[i]$ 时，$j=2$ 和 $j=3$ 的情况一定优于 $j \ge 4$ 的情况，因此无论是考虑 $j \times \text{dp}[i-j]$ 还是考虑 $j \times (i-j)$，都只需要考虑 $j=2$ 和 $j=3$ 的情况。

由此可以对方法一的动态规划进行优化。

边界情况是 $n=2$，此时唯一的拆分方案是 $2=1+1$，最大乘积是 $1 \times 1=1$。

当 $i \ge 3$ 时，状态转移方程如下：

$$
\text{dp}[i]=\max(2 \times (i-2), 2 \times \text{dp}[i-2], 3 \times (i-3), 3 \times \text{dp}[i-3])
$$

```go
func integerBreak(n int) int {
    if n < 4 {
        return n - 1
    }
    dp := make([]int, n + 1)
    dp[2] = 1
    for i := 3; i <= n; i++ {
        dp[i] = max(max(2 * (i - 2), 2 * dp[i - 2]), max(3 * (i - 3), 3 * dp[i - 3]))
    }
    return dp[n]
}

func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 $n$ 是给定的正整数。和方法一相比，计算每个整数对应的 $\text{dp}$ 的值的时间复杂度降到 $O(1)$，因此总时间复杂度降到 $O(n)$。
- 空间复杂度：$O(n)$，其中 $n$ 是给定的正整数。创建一个数组 $\text{dp}$，其长度为 $n+1$。


### 方法三：数学

> 不复制了，太多了。