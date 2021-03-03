- [303. 区域和检索 - 数组不可变](#303-区域和检索---数组不可变)
- [304. 二维区域和检索 - 矩阵不可变](#304-二维区域和检索---矩阵不可变)
- [338. 比特位计数](#338-比特位计数)

------------------------------

# 303. 区域和检索 - 数组不可变

给定一个整数数组  nums，求出数组从索引 i 到 j（i ≤ j）范围内元素的总和，包含 i、j 两点。

实现 NumArray 类：

- `NumArray(int[] nums)` 使用数组 nums 初始化对象
- `int sumRange(int i, int j)` 返回数组 nums 从索引 i 到 j（i ≤ j）范围内元素的总和，包含 i、j 两点（也就是 sum(`nums[i]`, `nums[i + 1]`, ... , `nums[j]`)）

示例：

```
输入：
["NumArray", "sumRange", "sumRange", "sumRange"]
[[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
输出：
[null, 1, -1, -3]

解释：
NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
numArray.sumRange(0, 2); // return 1 ((-2) + 0 + 3)
numArray.sumRange(2, 5); // return -1 (3 + (-5) + 2 + (-1)) 
numArray.sumRange(0, 5); // return -3 ((-2) + 0 + 3 + (-5) + 2 + (-1))
```

提示：

- 0 <= nums.length <= 104
- -105 <= `nums[i]` <= 105
- 0 <= i <= j < nums.length
- 最多调用 104 次 sumRange 方法

链接：https://leetcode-cn.com/problems/range-sum-query-immutable

**官方题解**

![](assets/0303_range-sum-query-immutable1.png)

```go
type NumArray struct {
    sums []int
}

func Constructor(nums []int) NumArray {
    sums := make([]int, len(nums)+1)
    for i, v := range nums {
        sums[i+1] = sums[i] + v
    }
    return NumArray{sums}
}

func (na *NumArray) SumRange(i, j int) int {
    return na.sums[j+1] - na.sums[i]
}
```

链接：https://leetcode-cn.com/problems/range-sum-query-immutable/solution/qu-yu-he-jian-suo-shu-zu-bu-ke-bian-by-l-px41/


# 304. 二维区域和检索 - 矩阵不可变

给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (row1, col1) ，右下角为 (row2, col2) 。

![](assets/0304_range-sum-query-2d-immutable.png)

上图子矩阵左上角 (row1, col1) = (2, 1) ，右下角(row2, col2) = (4, 3)，该子矩形内元素的总和为 8。

示例：

```
给定 matrix = [
  [3, 0, 1, 4, 2],
  [5, 6, 3, 2, 1],
  [1, 2, 0, 1, 5],
  [4, 1, 0, 1, 7],
  [1, 0, 3, 0, 5]
]

sumRegion(2, 1, 4, 3) -> 8
sumRegion(1, 1, 2, 2) -> 11
sumRegion(1, 2, 2, 4) -> 12
```

提示：

- 你可以假设矩阵不可变。
- 会多次调用 sumRegion 方法。
- 你可以假设 row1 ≤ row2 且 col1 ≤ col2 。

链接：https://leetcode-cn.com/problems/range-sum-query-2d-immutable

**官方题解**

前言

这道题是「303. 区域和检索 - 数组不可变」的进阶，第 303 题是在一维数组中做区域和检索，这道题是在二维矩阵中做区域和检索。

这道题有两种解法，分别是对每一行计算一维前缀和，以及对整个矩阵计算二维前缀和。

方法一：一维前缀和

第 303 题中，初始化时对数组计算前缀和，每次检索即可在 $O(1)$ 的时间内得到结果。可以将第 303 题的做法应用于这道题，初始化时对矩阵的每一行计算前缀和，检索时对二维区域中的每一行计算子数组和，然后对每一行的子数组和计算总和。

具体实现方面，创建 m 行 n+1 列的二维数组 sums，其中 m 和 n 分别是矩阵 matrix 的行数和列数，$\textit{sums}[i]$ 为 $\textit{matrix}[i]$ 的前缀和数组。将 sums 的列数设为 n+1 的目的是为了方便计算每一行的子数组和，不需要对 $\textit{col}_1=0$ 的情况特殊处理。

```go
type NumMatrix struct {
    sums [][]int
}

func Constructor(matrix [][]int) NumMatrix {
    sums := make([][]int, len(matrix))
    for i, row := range matrix {
        sums[i] = make([]int, len(row)+1)
        for j, v := range row {
            sums[i][j+1] = sums[i][j] + v
        }
    }
    return NumMatrix{sums}
}

func (nm *NumMatrix) SumRegion(row1, col1, row2, col2 int) (sum int) {
    for i := row1; i <= row2; i++ {
        sum += nm.sums[i][col2+1] - nm.sums[i][col1]
    }
    return
}
```

复杂度分析

- 时间复杂度：初始化 $O(mn)$，每次检索 $O(m)$，其中 m 和 n 分别是矩阵 matrix 的行数和列数。

    初始化需要遍历矩阵 matrix 计算二维前缀和，时间复杂度是 $O(mn)$。
    每次检索需要对二维区域中的每一行计算子数组和，二维区域的行数不超过 m，计算每一行的子数组和的时间复杂度是 $O(1)$，因此每次检索的时间复杂度是 $O(m)$。

- 空间复杂度：$O(mn)$，其中 m 和 n 分别是矩阵 matrix 的行数和列数。需要创建一个 m 行 n+1 列的前缀和数组 sums。

![](assets/0304_range-sum-query-2d-immutable1.png)

```go
type NumMatrix struct {
    sums [][]int
}

func Constructor(matrix [][]int) NumMatrix {
    m := len(matrix)
    if m == 0 {
        return NumMatrix{}
    }
    n := len(matrix[0])
    sums := make([][]int, m+1)
    sums[0] = make([]int, n+1)
    for i, row := range matrix {
        sums[i+1] = make([]int, n+1)
        for j, v := range row {
            // 上边 + 左边 - 左上 + 当前元素
            sums[i+1][j+1] = sums[i+1][j] + sums[i][j+1] - sums[i][j] + v
        }
    }
    return NumMatrix{sums}
}

func (nm *NumMatrix) SumRegion(row1, col1, row2, col2 int) int {
    return nm.sums[row2+1][col2+1] - nm.sums[row1][col2+1] - nm.sums[row2+1][col1] + nm.sums[row1][col1]
}
```

复杂度分析

- 时间复杂度：初始化 $O(mn)$，每次检索 $O(1)$，其中 m 和 n 分别是矩阵 matrix 的行数和列数。

    初始化需要遍历矩阵 matrix 计算二维前缀和，时间复杂度是 $O(mn)$。
    每次检索的时间复杂度是 $O(1)$。

- 空间复杂度：$O(mn)$，其中 m 和 n 分别是矩阵 matrix 的行数和列数。需要创建一个 m+1 行 n+1 列的二维前缀和数组 sums。

链接：https://leetcode-cn.com/problems/range-sum-query-2d-immutable/solution/er-wei-qu-yu-he-jian-suo-ju-zhen-bu-ke-b-2z5n/


# 338. 比特位计数

给定一个非负整数 num。对于 0 ≤ i ≤ num 范围中的每个数字 i ，计算其二进制数中的 1 的数目并将它们作为数组返回。

示例 1:

```
输入: 2
输出: [0,1,1]
```

示例 2:

```
输入: 5
输出: [0,1,1,2,1,2]
```

进阶:

- 给出时间复杂度为 $O(n*sizeof(integer))$ 的解答非常容易。但你可以在线性时间 $O(n)$ 内用一趟扫描做到吗？
- 要求算法的空间复杂度为 $O(n)$。
- 你能进一步完善解法吗？要求在C++或任何其他语言中不使用任何内置函数（如 C++ 中的 `__builtin_popcount`）来执行此操作。

链接：https://leetcode-cn.com/problems/counting-bits


**官方题解**

前言

这道题需要计算从 0 到 num 的每个数的二进制表示中的 1 的数目。最直观的方法是对每个数直接计算二进制表示中的 1 的数目，时间复杂度较高。也可以使用动态规划的方法，时间复杂度较低。

为了表述简洁，下文用「一比特数」表示二进制表示中的 1 的数目。

方法一：直接计算

最直观的方法是对从 0 到 num 的每个数直接计算「一比特数」。

每个 int 型的数都可以用 32 位二进制数表示，只要遍历其二进制表示的每一位即可得到 1 的数目。

利用位运算的技巧，可以在一定程度上提升计算速度。按位与运算（&）的一个性质是：对于任意整数 x，令 `x=x&(x−1)`，该运算将 x 的二进制表示的最后一个 1 变成 0。因此，对 x 重复该操作，直到 x 变成 0，则操作次数即为 x 的「一比特数」。

另外，部分编程语言有相应的内置函数，例如 Java 的 `Integer.bitCount`，C++ 的 `__builtin_popcount`，Go 的 `bits.OnesCount` 等，读者可以自行尝试。需要注意的是，使用编程语言的内置函数时，不适用本方法的时间复杂度分析。

```go
func onesCount(x int) (ones int) {
    for ; x > 0; x &= x - 1 {
        ones++
    }
    return
}

func countBits(num int) []int {
    bits := make([]int, num+1)
    for i := range bits {
        bits[i] = onesCount(i)
    }
    return bits
}
```

复杂度分析

- 时间复杂度：$O(k \times \textit{num})$，其中 k 是 int 型的二进制位数，k=32。需要对从 0 到 num 的每个数使用 $O(k)$ 的时间计算「一比特数」，因此时间复杂度是 $O(k \times \textit{num})$。
- 空间复杂度：$O(1)$。除了返回的数组以外，空间复杂度为常数。

--------------------

方法二：动态规划——最高有效位

方法一需要对每个数遍历其二进制表示的每一位。可以换一个思路，当计算 i 的「一比特数」时，如果存在 $0 \le j<i$，j 的「一比特数」已知，且 i 和 j 相比，i 的二进制表示只多了一个 1，则可以快速得到 i 的「一比特数」。

令 `bits[i]` 表示 i 的「一比特数」，则上述关系可以表示成：`bits[i]=bits[j]+1`。

对于正整数 x，如果可以知道最大的正整数 y，使得 $y \le x$ 且 y 是 2 的整数次幂，则 y 的二进制表示中只有最高位是 1，其余都是 0，此时称 y 为 x 的「最高有效位」。令 `z=x−y`，显然 $0 \le z<x$，则 `bits[x]=bits[z]+1`。(就是从 x 中把最高的 1 去掉后)

为了判断一个正整数是不是 2 的整数次幂，可以利用方法一中提到的按位与运算的性质。如果正整数 y 是 2 的整数次幂，则 y 的二进制表示中只有最高位是 1，其余都是 0，因此 `y&(y−1)=0`。由此可见，正整数 y 是 2 的整数次幂，当且仅当 `y&(y−1)=0`。

显然，0 的「一比特数」为 0。使用 highBit 表示当前的最高有效位，遍历从 1 到 num 的每个正整数 i，进行如下操作。

- 如果 `i&(i−1)=0`，则令 `highBit=i`，更新当前的最高有效位。
- i 比 i−highBit 的「一比特数」多 1，由于是从小到大遍历每个数，因此遍历到 i 时，i−highBit 的「一比特数」已知，令 `bits[i]=bits[i−highBit]+1`。

最终得到的数组 bits 即为答案。

```go
func countBits(num int) []int {
    bits := make([]int, num+1)
    highBit := 0
    for i := 1; i <= num; i++ {
        if i&(i-1) == 0 {
            highBit = i
        }
        bits[i] = bits[i-highBit] + 1
    }
    return bits
}
```

复杂度分析

- 时间复杂度：$O(\textit{num})$。对于每个数，只需要 $O(1)$ 的时间计算「一比特数」。
- 空间复杂度：$O(1)$。除了返回的数组以外，空间复杂度为常数。

--------------------

方法三：动态规划——最低有效位

方法二需要实时维护最高有效位，当遍历到的数是 2 的整数次幂时，需要更新最高有效位。如果再换一个思路，可以使用「最低有效位」计算「一比特数」。

对于正整数 x，将其二进制表示右移一位，等价于将其二进制表示的最低位去掉，得到的数是 $\lfloor \frac{x}{2} \rfloor$。如果 $\textit{bits}\big[\lfloor \frac{x}{2} \rfloor\big]$ 的值已知，则可以得到 $\textit{bits}[x]$ 的值：

- 如果 x 是偶数，则 $\textit{bits}[x]=\textit{bits}\big[\lfloor \frac{x}{2} \rfloor\big]$；
- 如果 x 是奇数，则 $\textit{bits}[x]=\textit{bits}\big[\lfloor \frac{x}{2} \rfloor\big]+1$。

上述两种情况可以合并成：$\textit{bits}[x]$ 的值等于 $\textit{bits}\big[\lfloor \frac{x}{2} \rfloor\big]$ 的值加上 x 除以 2 的余数。

由于 $\lfloor \frac{x}{2} \rfloor$ 可以通过 $x >> 1$ 得到，x 除以 2 的余数可以通过 $x \& 1$ 得到，因此有：$\textit{bits}[x]=\textit{bits}[x>>1]+(x \& 1)$。

遍历从 1 到 num 的每个正整数 i，计算 bits 的值。最终得到的数组 bits 即为答案。

```go
func countBits(num int) []int {
    bits := make([]int, num+1)
    for i := 1; i <= num; i++ {
        bits[i] = bits[i>>1] + i&1
    }
    return bits
}
```

复杂度分析

- 时间复杂度：$O(\textit{num})$。对于每个数，只需要 $O(1)$ 的时间计算「一比特数」。
- 空间复杂度：$O(1)$。除了返回的数组以外，空间复杂度为常数。

--------------------

方法四：动态规划——最低设置位

定义正整数 x 的「最低设置位」为 x 的二进制表示中的最低的 1 所在位。例如，10 的二进制表示是 $1010_{(2)}$​，其最低设置位为 2，对应的二进制表示是 $10_{(2)}$。

令 $y=x \&(x-1)$，则 y 为将 x 的最低设置位从 1 变成 0 之后的数，显然 $0 \le y<x$，`bits[x]=bits[y]+1`。因此对任意正整数 x，都有 `bits[x]=bits[x&(x−1)]+1`。

遍历从 1 到 num 的每个正整数 i，计算 bits 的值。最终得到的数组 bits 即为答案。

```go
func countBits(num int) []int {
    bits := make([]int, num+1)
    for i := 1; i <= num; i++ {
        bits[i] = bits[i&(i-1)] + 1
    }
    return bits
}
```

复杂度分析

- 时间复杂度：$O(\textit{num})$。对于每个数，只需要 $O(1)$ 的时间计算「一比特数」。
- 空间复杂度：$O(1)$。除了返回的数组以外，空间复杂度为常数。

链接：https://leetcode-cn.com/problems/counting-bits/solution/bi-te-wei-ji-shu-by-leetcode-solution-0t1i/

--------------------

> 下面这个题解对应的是上面的方法三。

对于所有的数字，只有两类：

奇数：二进制表示中，奇数一定比前面那个偶数多一个 1，因为多的就是最低位的 1。

```
0 = 0       1 = 1
2 = 10      3 = 11
```

偶数：二进制表示中，偶数中 1 的个数一定和除以 2 之后的那个数一样多。因为最低位是 0，除以 2 就是右移一位，也就是把那个 0 抹掉而已，所以 1 的个数是不变的。

```
2 = 10       4 = 100       8 = 1000
3 = 11       6 = 110       12 = 1100
```

另外，0 的 1 个数为 0，于是就可以根据奇偶性开始遍历计算了。

```c++
vector<int> countBits(int num) {
    vector<int> result(num+1);
    result[0] = 0;
    for(int i = 1; i <= num; i++)
    {
        if(i % 2 == 1)
        {
            result[i] = result[i-1] + 1;
        }
        else
        {
            result[i] = result[i/2];
        }
    }
    
    return result;
}
```

链接：https://leetcode-cn.com/problems/counting-bits/solution/hen-qing-xi-de-si-lu-by-duadua/