- [867. 转置矩阵](#867-转置矩阵)


------------------------------

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