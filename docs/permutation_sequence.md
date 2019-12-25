# permutation_sequence

The set `[1,2,3,…,*n*]` contains a total of *n*! unique permutations.

By listing and labeling all of the permutations in order,
We get the following sequence (ie, for *n* = 3):

1. `"123"`
2. `"132"`
3. `"213"`
4. `"231"`
5. `"312"`
6. `"321"`

Given *n* and *k*, return the *k*th permutation sequence.

**Note:** Given *n* will be between 1 and 9 inclusive.



<https://discuss.leetcode.com/topic/17348/explain-like-i-m-five-java-solution-in-o-n>



假设 n=4, 那么我们就有 `{1,2,3,4}`

所有的排列如下：

```
1 + (permutations of 2, 3, 4)
2 + (permutations of 1, 3, 4)
3 + (permutations of 1, 2, 4)
4 + (permutations of 1, 2, 3)
```

n个数的排列一共有`n!`种，3个数的排列一共有6中可能，所以上面一共是24种可能。如果我们要找第k个排列（假设k=14）,那么它应该是在`3 + (permutations of 1, 2, 4)`中。

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



