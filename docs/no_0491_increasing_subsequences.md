- [491. 递增子序列](#491-递增子序列)
  - [官方题解](#官方题解)
    - [方法一：二进制枚举 + 哈希](#方法一二进制枚举--哈希)
    - [方法二：递归枚举 + 减枝](#方法二递归枚举--减枝)

------------------------------

# 491. 递增子序列

给定一个整型数组, 你的任务是找到所有该数组的递增子序列，递增子序列的长度至少是2。

示例:

```
输入: [4, 6, 7, 7]
输出: [[4, 6], [4, 7], [4, 6, 7], [4, 6, 7, 7], [6, 7], [6, 7, 7], [7,7], [4,7,7]]
```

说明:

1. 给定数组的长度不会超过15。
2. 数组中的整数范围是 `[-100,100]`。
3. 给定数组中可能包含**重复数字**，相等的数字应该被视为递增的一种情况。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/increasing-subsequences
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

> 我能想到的是先排序。
> 排序不行，假设输入是 `[3, 2, 1]`，那没有子序列是递增的，但排序后就有了，这肯定不行。

## 官方题解

> 链接：https://leetcode-cn.com/problems/increasing-subsequences/solution/di-zeng-zi-xu-lie-by-leetcode-solution/

### 方法一：二进制枚举 + 哈希

**思路与算法**

我们可以采取最朴素的思路，即**枚举出所有的子序列**，然后判断当前的子序列是否是非严格递增的。那么我们可以用什么办法来枚举所有的子序列呢？我们需要从原序列中选出一些数，来形成新的序列，即原序列的子序列。对于原序列的每一个数来说，都有两种可能的状态，即被选中或者不被选中。如果我们用 1 代表被选中，0 代表不被选中，假设一个序列长度为 3，那么选出的子序列就对应着下面的八种状态：

- `[0, 0, 0]`
- `[0, 0, 1]`
- `[0, 1, 0]`
- `[0, 1, 1]`
- `[1, 0, 0]`
- `[1, 0, 1]`
- `[1, 1, 0]`
- `[1, 1, 1]`

由此可见长度为 n 的序列选择子序列一共会有 $2^{n}$ 种情况，这 $2^{n}$ 种情况就是区间 $[0， 2^{n - 1}]$ 的所有整数的**二进制表示**。我们可以枚举区间 $[0， 2^{n - 1}]$ 中的每一个数，然后对它做二进制数位拆分，我们会得到一个 $0/1$ 序列，接着可以构造出这个 $0/1$ 序列对应的子序列，然后再检查这个序列是否是非严格递增的。

当然，我们还需要解决子序列**去重**的问题。对于序列去重，我们可以使用串哈希算法（即 Rabin-Karp 编码，这里不了解的同学可以参考 「[官方题解 - 1392. 最长快乐前缀](https://leetcode-cn.com/problems/longest-happy-prefix/solution/zui-chang-kuai-le-qian-zhui-by-leetcode-solution/)」），即对于一个序列 $a_0, a_1, \cdots, a_{n - 1}$，我们可以认为是一个 $\max \{ a_i \} + 1$ 进制的数，这个数的数值等于（记 $b = \max \{ a_i \} + 1$：

$$
f(\vec{a}) = \sum_{i = 0}^{n - 1} b^i \times a_i
$$

每次我们找到一个合法序列的时候，都去计算这个序列的哈希值，用一个哈希表来记录已有的哈希值，如果该值已经出现在哈希表中，就舍弃这个序列，否则就把这个序列加入到答案中。

在实现的过程中，我们发现这个哈希值可能非常大，我们可以将它模一个大素数 $P$，将这个值映射到 $\rm int$ 的范围内。所以实际上这里的哈希函数是：

$$
f(\vec{a}) = \sum_{i = 0}^{n - 1} b^i \times a_i \pmod P
$$

而这里的 $b$ 也未必是 $\max \{ a_i \} + 1$，它可以任意选一个大于 $\max \{ a_i \} + 1$ 的数。

```go
var (
    n int
    temp []int
)
func findSubsequences(nums []int) [][]int {
    n = len(nums)
    ans := [][]int{}
    set := map[int]bool{}
    for i := 0; i < 1 << n; i++ {
        findSubsequences1(i, nums)
        // 263 进制，263 >= 101 就行。
        // 1e10 + 7 是那个大素数。
        hashValue := getHash(263, int(1e9 + 7))
        if check() && !set[hashValue] {
            t := make([]int, len(temp))
            copy(t, temp)
            ans = append(ans, t)
            set[hashValue] = true
        }
    }
    return ans
}

func findSubsequences1(mask int, nums []int) {
    // 如果 mask 二进制的第 i 位为 1，就把 nums[i] 加入到 temp 中。
    temp = []int{}
    for i := 0; i < n; i++ {
        if (mask & 1) != 0 {
            temp = append(temp, nums[i])
        }
        mask >>= 1
    }
}

func getHash(base, mod int) int {
    hashValue := 0
    for _, x := range temp {
        // 跟从二进制计算十进制一样，n = n * 10 + x;
        // x + 101 是为了保证是正数？
        hashValue = hashValue * base % mod + (x + 101)
        hashValue %= mod
    }
    return hashValue
}

func check() bool {
    // 是否是递增的序列。
    for i := 1; i < len(temp); i++ {
        if temp[i] < temp[i - 1] {
            return false
        }
    }
    return len(temp) >= 2
}
```

**复杂度分析**

假设序列的长度是 $n$。

- 时间复杂度：$O(2^n \cdot n)$。这里枚举所有子序列的时间代价是 $O(2^n)$，每次检测序列是否合法和获取哈希值的时间代价都是 $O(n)$，故渐近时间复杂度为 $O(2^n \cdot n)$。
- 空间复杂度：$O(2^n)$。最坏情况下整个序列都是递增的，每个长度大于等于 2 的子序列都要加入答案，这里哈希表中要加入 $2^n$ 个元素，空间代价为 $O(2^n)$；用一个临时的数组来存当前答案，空间代价为 $O(n)$。

### 方法二：递归枚举 + 减枝

**思路与算法**

我们也可以用递归的方法实现二进制枚举，像「方法一」那样枚举出所有的子序列，然后判断是否合法。直接把方法一变成递归形式，我们可以得到这样的代码：

```java
List<List<Integer>> ans = new ArrayList<List<Integer>>();
List<Integer> temp = new ArrayList<Integer>();
public void dfs(int cur, int[] nums) {
    if (cur == nums.length) {
        // 判断是否合法，如果合法判断是否重复，将满足条件的加入答案
        if (isValid() && notVisited()) {
            ans.add(new ArrayList<Integer>(temp));
        }
        return;
    }
    // 如果选择当前元素
    temp.add(nums[cur]);
    dfs(cur + 1, nums);
    temp.remove(temp.size() - 1);
    // 如果不选择当前元素
    dfs(cur + 1, nums);
}
```

这是一个递归枚举子序列的通用模板，即用一个临时数组 $\rm temp$ 来保存当前选出的子序列，使用 $\rm cur$ 来表示当前位置的下标，在 `dfs(cur, num)` 开始之前，$[0, {\rm cur} - 1]$ 这个区间内的所有元素都已经被考虑过，而 $[{\rm cur}, n]$ 这个区间内的元素还未被考虑。在执行 `dfs(cur, num)` 时，我们考虑 ${\rm cur}$ 这个位置选或者不选，如果选择当前元素，那么把当前元素加入到 $\rm temp$ 中，然后递归下一个位置，在递归结束后，应当把 $\rm temp$ 的最后一个元素删除进行回溯；如果不选当前的元素，直接递归下一个位置。

当然，如果我们简单地这样枚举，对于每一个子序列，我们还需要做一次 $O(n)$ 的合法性检查和哈希判重复，在执行整个程序的过程中，我们还需要使用一个空间代价 $O(2^n)$ 的哈希表来维护已经出现的子序列的哈希值。我们可以对选择和不选择做一些简单的限定，就可以让枚举出来的都是合法的并且不重复：

- 使序列合法的办法非常简单，即给「选择」做一个限定条件，**只有当前的元素大于等于上一个选择的元素的时候才能选择这个元素**，这样枚举出来的所有元素都是合法的
- 那如何保证没有重复呢？我们需要给「不选择」做一个限定条件，**只有当当前的元素不等于上一个选择的元素的时候**，才考虑不选择当前元素，直接递归后面的元素。因为如果有两个相同的元素，我们会考虑这样四种情况：
    1. 前者被选择，后者被选择
    2. 前者被选择，后者不被选择
    3. 前者不被选择，后者被选择
    4. 前者不被选择，后者不被选择

其中第二种情况和第三种情况其实是等价的，我们这样限制之后，舍弃了第二种，保留了第三种，于是达到了去重的目的。

```go
var (
    temp []int
    ans [][]int
)

func findSubsequences(nums []int) [][]int {
    ans = [][]int{}
    dfs(0, math.MinInt32, nums)
    return ans
}

func dfs(cur, last int, nums []int) {
    if cur == len(nums) {
        if len(temp) >= 2 {
            t := make([]int, len(temp))
            copy(t, temp)
            ans = append(ans, t)
        }
        return
    }
    if nums[cur] >= last {
        temp = append(temp, nums[cur])
        dfs(cur + 1, nums[cur], nums)
        temp = temp[:len(temp)-1]
    }
    if nums[cur] != last {
        dfs(cur + 1, last, nums)
    }
}
```

**复杂度分析**

- 时间复杂度：$O(2^n \cdot n)$。仍然需要对子序列做二进制枚举，枚举出的序列虽然省去了判断合法性和哈希的过程，但是仍然需要 $O(n)$ 的时间添加到答案中。
- 空间复杂度：$O(n)$。这里临时数组的空间代价是 $O(n)$，递归使用的栈空间的空间代价也是 $O(n)$。
