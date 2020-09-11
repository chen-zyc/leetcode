- [216. 组合总和 III](#216-组合总和-iii)
	- [官方题解](#官方题解)
		- [方法一：二进制（子集）枚举](#方法一二进制子集枚举)
		- [方法二：组合枚举](#方法二组合枚举)
	- [以前的题解](#以前的题解)




# 216. 组合总和 III

找出所有相加之和为 n 的 k 个数的组合。组合中只允许含有 1 - 9 的正整数，并且每种组合中不存在重复的数字。

说明：

- 所有数字都是正整数。
- 解集不能包含重复的组合。 

示例 1:

```
输入: k = 3, n = 7
输出: [[1,2,4]]
```

示例 2:

```
输入: k = 3, n = 9
输出: [[1,2,6], [1,3,5], [2,3,4]]
```


- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/combination-sum-iii
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/combination-sum-iii/solution/zu-he-zong-he-iii-by-leetcode-solution/

### 方法一：二进制（子集）枚举

**思路与算法**

「组合中只允许含有 1-9 的正整数，并且每种组合中不存在重复的数字」意味着这个组合中最多包含 9 个数字。我们可以把原问题转化成集合 `S={1,2,3,4,5,6,7,8,9}`，我们要找出 S 的当中满足如下条件的子集：

- 大小为 k
- 集合中元素的和为 n

因此我们可以用子集枚举的方法来做这道题。即原序列中有 9 个数，每个数都有两种状态，「被选择到子集中」和「不被选择到子集中」，所以状态的总数为 $2^9$。我们用一个 9 位二进制数 `mask` 来记录当前所有位置的状态，从低到高第 i 位为 0 表示 i 不被选择到子集中，为 1 表示 i 被选择到子集中。当我们按顺序枚举 $[0, 2^9 - 1]$ 中的所有整数的时候，就可以不重不漏地把每个状态枚举到，对于一个状态 `mask`，我们可以用位运算的方法得到对应的子集序列，然后再判断是否满足上面的两个条件，如果满足，就记录答案。

如何通过位运算来得到 `mask` 各个位置的信息？对于第 i 个位置我们可以判断 $(1 << i) & mask$ 是否为 0，如果不为 0 则说明 i 在子集当中。当然，这里要注意的是，一个 9 位二进制数 i 的范围是 $[0, 8]$，而可选择的数字是 $[1, 9]$，所以我们需要做一个映射，最简单的办法就是当我们知道 i 位置不为 0 的时候将 i + 1 加入子集。

当然，子集枚举也可以用递归实现。在「[官方题解 - 77. 组合](https://leetcode-cn.com/problems/combinations/solution/zu-he-by-leetcode-solution/)」的方法一中提及了子集枚举递归实现的基本框架，感兴趣的同学可以参考。

```go
func combinationSum3(k int, n int) (ans [][]int) {
	var temp []int
	check := func(mask int) bool {
		temp = nil
		sum := 0
		for i := 0; i < 9; i++ {
			if 1<<i&mask > 0 {
				temp = append(temp, i+1)
				sum += i + 1
			}
		}
		return len(temp) == k && sum == n
	}

	for mask := 0; mask < 1<<9; mask++ {
		if check(mask) {
			ans = append(ans, append([]int(nil), temp...))
		}
	}
	return
}
```

**复杂度分析**

- 时间复杂度：$O(M \times 2^M)$，其中 M 为集合的大小，本题中 M 固定为 9。一共有 $2^M$ 个状态，每个状态需要 $O(M + k) = O(M)$ 的判断 （$k \leq M$），故时间复杂度为 $O(M \times 2^M)$。
- 空间复杂度：$O(M)$。即 temp 的空间代价。



### 方法二：组合枚举

**思路与算法**

我们可以换一个思路：我们需要在 9 个数中选择 k 个数，让它们的和为 n。

这样问题就变成了一个组合枚举问题。组合枚举有两种处理方法——递归法和字典序法，在「[官方题解 - 77. 组合](https://leetcode-cn.com/problems/combinations/solution/zu-he-by-leetcode-solution/)」中有详细的说明。

这里我们要做的是做一个「在 9 个数中选择 k 个数」的组合枚举，对于枚举到的所有组合，判断这个组合内元素之和是否为 n。

```go
func combinationSum3(k int, n int) (ans [][]int) {
	var temp []int
	var dfs func(cur, rest int)
	dfs = func(cur, rest int) {
		// 找到一个答案
		if len(temp) == k && rest == 0 {
			ans = append(ans, append([]int(nil), temp...))
			return
		}
		// 剪枝：跳过的数字过多，后面已经无法选到 k 个数字
		if len(temp)+10-cur < k || rest < 0 {
			return
		}
		// 跳过当前数字
		dfs(cur+1, rest)
		// 选当前数字
		temp = append(temp, cur)
		dfs(cur+1, rest-cur)
		temp = temp[:len(temp)-1]
	}
	dfs(1, n)
	return
}
```

**复杂度分析**

- 时间复杂度：$O({M \choose k} \times k)$，其中 M 为集合的大小，本题中 M 固定为 9。一共有 $M \choose k$ 个组合，每次判断(复制答案的代价)需要的时间代价是 $O(k)$。
- 空间复杂度：$O(M)$。temp 数组的空间代价是 $O(k)$，递归栈空间的代价是 $O(M)$，故空间复杂度为 $O(M + k) = O(M)$.




## 以前的题解

```go
func CombinationSum3(k int, n int) [][]int {
	result := [][]int{}
	combinationSum3Help(k, n, 1, nil, &result)
	return result
}

func combinationSum3Help(k, n, start int, com []int, result *[][]int) {
	if n == 0 && k == 0 {
		comCopy := make([]int, len(com))
		copy(comCopy, com)
		*result = append(*result, comCopy)
		return
	}
	if k > 0 {
		for i := start; i <= 9; i++ {
			if n >= i {
				com = append(com, i)
				combinationSum3Help(k-1, n-i, i+1, com, result)
				com = com[:len(com)-1]
				continue
			}
			return
		}
	}
}
```

思路和 [combination_sum.md](doc/combination_sum.md) 及 [combination_sum_2.md](doc/combination_sum_2.md) 基本一致。

