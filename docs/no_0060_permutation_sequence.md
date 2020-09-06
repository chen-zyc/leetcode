- [60. 第k个排列](#60-第k个排列)
  - [官方题解](#官方题解)
    - [方法一：数学 + 缩小问题规模](#方法一数学--缩小问题规模)
  - [以前的题解](#以前的题解)

------------------------------

# 60. 第k个排列

给出集合 `[1,2,3,…,n]`，其所有元素共有 `n!` 种排列。

按大小顺序列出所有排列情况，并一一标记，当 n = 3 时, 所有排列如下：

```
"123"
"132"
"213"
"231"
"312"
"321"
```

给定 n 和 k，返回第 k 个排列。

说明：

- 给定 n 的范围是 `[1, 9]`。
- 给定 k 的范围是 `[1,  n!]`。

示例 1:

```
输入: n = 3, k = 3
输出: "213"
```

示例 2:

```
输入: n = 4, k = 9
输出: "2314"
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/permutation-sequence
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/permutation-sequence/solution/di-kge-pai-lie-by-leetcode-solution/
> 官方题解太公式化了，不形象。

### 方法一：数学 + 缩小问题规模

**思路**

要想解决本题，首先需要了解一个简单的结论：

对于 n 个不同的元素（例如数 $1, 2, \cdots, n$），它们可以组成的排列总数目为 $n!$。

> 对于给定的 n 和 k，我们不妨从左往右确定第 k 个排列中的每一个位置上的元素到底是什么。

我们首先确定排列中的首个元素 $a_1$。根据上述的结论，我们可以知道：

- 以 1 为 $a_1$ 的排列一共有 $(n-1)!$ 个；
- 以 2 为 $a_1$ 的排列一共有 $(n-1)!$ 个；
- $\cdots$
- 以 n 为 $a_1$ 的排列一共有 $(n-1)!$ 个。

由于我们需要求出从小到大的第 k 个排列，因此：

- 如果 $k \leq (n-1)!$，我们就可以确定排列的首个元素为 1；
- 如果 $(n-1)! < k \leq 2 \cdot (n-1)!$，我们就可以确定排列的首个元素为 2；
- $\cdots$
- 如果 $(n-1) \cdot (n-1)! < k \leq n \cdot (n-1)!$，我们就可以确定排列的首个元素为 n。

因此，第 k 个排列的首个元素就是：

$$
a_1 = \lfloor \frac{k-1}{(n-1)!} \rfloor + 1
$$

> 这是为什么是 k-1，最后又为什么要 +1？
> 其实就是 k/(n-1)!.

其中 $\lfloor x \rfloor$ 表示将 x 向下取整。

当我们确定了 $a_1$ 后，如何使用相似的思路，确定下一个元素 $a_2$ 呢？实际上，我们考虑以 $a_1$ 为首个元素的所有排列：

- 以 $[1,n] \backslash a_1$ 最小的元素为 $a_2$ 的排列一共有 $(n-2)!$ 个；
- 以 $[1,n] \backslash a_1$ 次小的元素为 $a_2$ 的排列一共有 $(n-2)!$ 个；
- $\cdots$ 
- 以 $[1,n] \backslash a_1$ 最大的元素为 $a_2$ 的排列一共有 $(n-2)!$ 个；

其中 $[1,n] \backslash a_1$ 表示包含 $1, 2, \cdots n$ 中除去 $a_1$ 以外元素的集合。这些排列从编号 $(a_1-1) \cdot (n-1)!$ 开始，到 $a_1 \cdot (n-1)!$ 结束，总计 $(n-1)!$ 个，因此第 k 个排列实际上就对应着这其中的第

$$
k' = (k-1) \bmod (n-1)! + 1
$$

个排列。这样一来，我们就把原问题转化成了一个完全相同但规模减少 1 的子问题：

> 求 $[1, n] \backslash a_1$ 这 $n-1$ 个元素组成的排列中，第 $k'$ 小的排列。

> 说人话就是，a1 已经确认了，从剩下的元素集合中再选第 k' 个排列。

**算法**

设第 k 个排列为 $a_1, a_2, \cdots, a_n$，我们从左往右地确定每一个元素 $a_i$ 。我们用数组 $\textit{valid}$ 记录每一个元素是否被使用过。

我们从小到大枚举 i：

- 我们已经使用过了 $i-1$ 个元素，剩余 $n-i+1$ 个元素未使用过，每一个元素作为 $a_i$ 都对应着 $(n-i)!$ 个排列，总计 $(n-i+1)!$ 个排列；
- 因此在第 k 个排列中，$a_i$ 即为剩余未使用过的元素中第 $\lfloor \frac{k-1}{(n-i)!} \rfloor + 1$ 小的元素；
- 在确定了 $a_i$ 后，这 $n-i+1$ 个元素的第 k 个排列，就等于 $a_i$ 之后跟着剩余 $n-i$ 个元素的第 $(k-1) \bmod (n-i)! + 1$ 个排列。

在实际的代码中，我们可以首先将 k 的值减少 1，这样可以减少运算，降低代码出错的概率。对应上述的后两步，即为：

- 因此在第 k 个排列中，$a_i$ 即为剩余未使用过的元素中第 $\lfloor \frac{k}{(n-i)!} \rfloor + 1$ 小的元素；
- 在确定了 $a_i$ 后，这 $n-i+1$ 个元素的第 k 个排列，就等于 $a_i$ 之后跟着剩余 $n-i$ 个元素的第 $k \bmod (n-i)!$ 个排列。

实际上，这相当于我们将所有的排列从 0 开始进行编号。

```go
func getPermutation(n int, k int) string {
    // 从 0 到 n - 1 的阶。
    factorial := make([]int, n)
    factorial[0] = 1
    for i := 1; i < n; i++ {
        factorial[i] = factorial[i - 1] * i
    }
    k--

    ans := ""
    // n 个数字都标记成未使用。
    valid := make([]int, n + 1)
    for i := 0; i < len(valid); i++ {
        valid[i] = 1
    }
    for i := 1; i <= n; i++ {
        // +1 可以被认为是 valid 中的 0 是没有用的。
        order := k / factorial[n - i] + 1
        for j := 1; j <= n; j++ {
            // 如果 valid[j] 是 1，order 减去它表示略过它，如果是 0 表示用过了，就不用略过了。
            // 这个可以避免从 valid 中删除 j 而导致的数组复制。
            order -= valid[j]
            if order == 0 {
                ans += strconv.Itoa(j)
                valid[j] = 0
                break
            }
        }
        k %= factorial[n - i]
    }
    return ans
}
```

**复杂度分析**

- 时间复杂度：$O(n^2)$。 
- 空间复杂度：$O(n)$。

**思考题**

对于给定的排列 $a_1, a_2, \cdots, a_n$，你能求出 k 吗？

解答：

$$
k = \left( \sum_{i=1}^n \textit{order}(a_i) \cdot (n-i)! \right) + 1
$$

其中 $\textit{order}(a_i)$ 表示 $a_{i+1}, \cdots, a_n$ 中小于 $a_i$ 的元素个数。


## 以前的题解

假设 n=4, 那么我们就有 `{1,2,3,4}`

所有的排列如下：

```
1 + (permutations of 2, 3, 4)
2 + (permutations of 1, 3, 4)
3 + (permutations of 1, 2, 4)
4 + (permutations of 1, 2, 3)
```

n个数的排列一共有`n!`种，3个数的排列一共有6种可能，所以上面一共是24种可能。如果我们要找第k个排列（假设k=14）,那么它应该是在`3 + (permutations of 1, 2, 4)`中。

用程序来计算就是这样的：k=13(减一是因为要从0开始)，`k/(n-1)! = 13/(4-1)! = 13/3! = 13/6 = 2`，所以第k个排列在`3 + (permutations of 1, 2, 4)`中，所以第一个数字是3.

继续在剩下的数字中重复。

`{1,2,4}`的排列如下：

```
1 + (permutations of 2, 4)
2 + (permutations of 1, 4)
4 + (permutations of 1, 2)
```

但是k不再是14了。因为在前面的步骤中我们排除了12种可能，所以我们需要在剩下的排列中找到第2个排列。用程序计算就是：

```
k = k - (index from previous) * (n-1)! = k - 2*(n-1)! = 13 - 2*(3)! = 1
```

第二步中，2个数字仅有2种排列。所以我们要在`1 + (permutations of 2, 4)`中找。

现在我们已经得到了序列`3,1`。继续`{2, 4}`的排列。

```
k = k - (index from pervious) * (n-2)! = k - 0 * (n - 2)! = 1 - 0 = 1;
```

从`{2,4}`的排列中找索引为1的排列，也就是第2个排列。

第三个数字的索引是`k / (n - 3)! = 1 / (4-3)! = 1/ 1! = 1`,所以第三个数字是4.

还剩下`{2}`，k更新为`k = k - (index from pervious) * (n - 3)! = k - 1 * (4 - 3)! = 1 - 1 = 0;`,索引是`k / (n - 4)! = 0 / (4-4)! = 0/ 1 = 0`，所以第4个数字是2.

最终的结果就是`3142`。



```go
func getPermutation(n int, k int) string {
	// 阶乘 [0!, 1!, 2!, 3!... n!]
	factorial := make([]int, n+1)
	sum := 1
	factorial[0] = 1
	for i := 1; i <= n; i++ {
		sum *= i
		factorial[i] = sum
	}

	// 数字列表 [1, 2, 3, ... n]
	numbers := make([]int, n)
	for i := 0; i < n; i++ {
		numbers[i] = i + 1
	}

	k-- // 从0开始

	res := make([]byte, n)
	for i := 1; i <= n; i++ {
		index := k / factorial[n-i]
		res[i-1] = byte('0' + numbers[index])                   // n在1-9之间
		numbers = append(numbers[:index], numbers[index+1:]...) // 删除被选中的数字
		k -= index * factorial[n-i]
	}
	return string(res)
}
```


