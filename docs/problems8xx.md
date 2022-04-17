- [819. 最常见的单词](#819-最常见的单词)
- [867. 转置矩阵](#867-转置矩阵)


------------------------------

# 819. 最常见的单词

给定一个段落 (paragraph) 和一个禁用单词列表 (banned)。返回出现次数最多，同时不在禁用列表中的单词。

题目保证至少有一个词不在禁用列表中，而且答案唯一。

禁用列表中的单词用小写字母表示，不含标点符号。段落中的单词不区分大小写。答案都是小写字母。

示例：

```
输入: 
paragraph = "Bob hit a ball, the hit BALL flew far after it was hit."
banned = ["hit"]
输出: "ball"
解释: 
"hit" 出现了3次，但它是一个禁用的单词。
"ball" 出现了2次 (同时没有其他单词出现2次)，所以它是段落里出现次数最多的，且不在禁用列表中的单词。 
注意，所有这些单词在段落里不区分大小写，标点符号需要忽略（即使是紧挨着单词也忽略， 比如 "ball,"）， 
"hit"不是最终的答案，虽然它出现次数更多，但它在禁用单词列表中。
```

提示：

- 1 <= 段落长度 <= 1000
- 0 <= 禁用单词个数 <= 100
- 1 <= 禁用单词长度 <= 10
- 答案是唯一的, 且都是小写字母 (即使在 paragraph 里是大写的，即使是一些特定的名词，答案都是小写的。)
- paragraph 只包含字母、空格和下列标点符号!?',;.
- 不存在没有连字符或者带有连字符的单词。
- 单词里只包含字母，不会出现省略号或者其他标点符号。

链接：https://leetcode-cn.com/problems/most-common-word

> 第一次：用 map 统计单词出现的次数，然后取出现次数最大的那个。


# 867. 转置矩阵

给你一个二维整数数组 matrix， 返回 matrix 的 转置矩阵 。

矩阵的 转置 是指将矩阵的主对角线翻转，交换矩阵的行索引与列索引。

![](assets/8xx_transpose-matrix.png)

示例 1：

```
输入：matrix = [[1,2,3],[4,5,6],[7,8,9]]
输出：[[1,4,7],[2,5,8],[3,6,9]]
```

示例 2：

```
输入：matrix = [[1,2,3],[4,5,6]]
输出：[[1,4],[2,5],[3,6]]
```

提示：

- `m == matrix.length`
- `n == matrix[i].length`
- 1 <= m, n <= 1000
- 1 <= m * n <= 105
- `-109 <= matrix[i][j] <= 109`

链接：https://leetcode-cn.com/problems/transpose-matrix


**官方题解**

如果矩阵 $\textit{matrix}$ 为 $m$ 行 $n$ 列，则转置后的矩阵 $\textit{matrix}^\text{T}$ 为 $n$ 行 $m$ 列，且对任意 $0 \le i<m$ 和 $0 \le j<n$，都有 $\textit{matrix}^\text{T}[j][i]=\textit{matrix}[i][j]$。

创建一个 $n$ 行 $m$ 列的新矩阵，根据转置的规则对新矩阵中的每个元素赋值，则新矩阵为转置后的矩阵。

```go
func transpose(matrix [][]int) [][]int {
    n, m := len(matrix), len(matrix[0])
    t := make([][]int, m)
    for i := range t {
        t[i] = make([]int, n)
        for j := range t[i] {
            t[i][j] = -1
        }
    }
    for i, row := range matrix {
        for j, v := range row {
            t[j][i] = v
        }
    }
    return t
}
```

- 时间复杂度：$O(mn)$，其中 $m$ 和 $n$ 分别是矩阵 $\textit{matrix}$ 的行数和列数。需要遍历整个矩阵，并对转置后的矩阵进行赋值操作。
- 空间复杂度：$O(1)$。除了返回值以外，额外使用的空间为常数。

链接：https://leetcode-cn.com/problems/transpose-matrix/solution/zhuan-zhi-ju-zhen-by-leetcode-solution-85s2/