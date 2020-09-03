- [51. N 皇后](#51-n-皇后)
  - [官方题解](#官方题解)
    - [方法一：基于集合的回溯](#方法一基于集合的回溯)
    - [方法二：基于位运算的回溯](#方法二基于位运算的回溯)


------------------------------

# 51. N 皇后

n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。

![](assets/no_0051_n_queens.png)

上图为 8 皇后问题的一种解法。

给定一个整数 n，返回所有不同的 n 皇后问题的解决方案。

每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。

 
示例：

```
输入：4
输出：[
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]
解释: 4 皇后问题存在两个不同的解法。
```

提示：

皇后彼此不能相互攻击，也就是说：任何两个皇后都不能处于同一条横行、纵行或斜线上。

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/n-queens
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。



## 官方题解

> 链接：https://leetcode-cn.com/problems/n-queens/solution/nhuang-hou-by-leetcode-solution/

**前言**

「N 皇后问题」研究的是如何将 N 个皇后放置在 N×N 的棋盘上，并且使皇后彼此之间不能相互攻击。

皇后的走法是：可以横直斜走，格数不限。因此要求皇后彼此之间不能相互攻击，等价于要求任何两个皇后都不能在同一行、同一列以及同一条斜线上。

直观的做法是暴力枚举将 N 个皇后放置在 N×N 的棋盘上的所有可能的情况，并对每一种情况判断是否满足皇后彼此之间不相互攻击。暴力枚举的时间复杂度是非常高的，因此必须利用限制条件加以优化。

显然，每个皇后必须位于不同行和不同列，因此将 N 个皇后放置在 N×N 的棋盘上，一定是每一行有且仅有一个皇后，每一列有且仅有一个皇后，且**任何两个皇后都不能在同一条斜线上**。基于上述发现，可以通过回溯的方式寻找可能的解。

回溯的具体做法是：使用一个数组记录每行放置的皇后的列下标，依次在每一行放置一个皇后。每次新放置的皇后都不能和已经放置的皇后之间有攻击：即新放置的皇后不能和任何一个已经放置的皇后在同一列以及同一条斜线上，并更新数组中的当前行的皇后列下标。当 N 个皇后都放置完毕，则找到一个可能的解。当找到一个可能的解之后，将数组转换成表示棋盘状态的列表，并将该棋盘状态的列表加入返回列表。

由于每个皇后必须位于不同列，因此已经放置的皇后所在的列不能放置别的皇后。第一个皇后有 N 列可以选择，第二个皇后最多有 N−1 列可以选择，第三个皇后最多有 N−2 列可以选择（如果考虑到不能在同一条斜线上，可能的选择数量更少），因此所有可能的情况不会超过 N! 种，遍历这些情况的时间复杂度是 $O(N!)$。

为了降低总时间复杂度，每次放置皇后时需要快速判断每个位置是否可以放置皇后，显然，最理想的情况是在 $O(1)$ 的时间内判断该位置所在的列和两条斜线上是否已经有皇后。

以下两种方法分别使用集合和位运算对皇后的放置位置进行判断，都可以在 $O(1)$ 的时间内判断一个位置是否可以放置皇后，算法的总时间复杂度都是 $O(N!)$。



### 方法一：基于集合的回溯

为了判断一个位置所在的列和两条斜线上是否已经有皇后，使用三个集合 $\textit{columns}$、$\textit{diagonals}_1$	和 $\textit{diagonals}_2$ 分别记录每一列以及两个方向的每条斜线上是否有皇后。

列的表示法很直观，一共有 N 列，每一列的下标范围从 0 到 N−1，使用列的下标即可明确表示每一列。

如何表示两个方向的斜线呢？对于每个方向的斜线，需要找到斜线上的每个位置的行下标与列下标之间的关系。

方向一的斜线为从左上到右下方向，**同一条斜线上的每个位置满足行下标与列下标之差相等**(这是怎么想出来的呢？)，例如 $(0,0)$ 和 $(3,3)$ 在同一条方向一的斜线上。因此使用行下标与列下标之差即可明确表示每一条方向一的斜线。

![](assets/no_0051_n_queens1.png)

方向二的斜线为从右上到左下方向，**同一条斜线上的每个位置满足行下标与列下标之和相等**，例如 $(3,0)$ 和 $(1,2)$ 在同一条方向二的斜线上。因此使用行下标与列下标之和即可明确表示每一条方向二的斜线。

![](assets/no_0051_n_queens2.png)

每次放置皇后时，对于每个位置判断其是否在三个集合中，如果三个集合都不包含当前位置，则当前位置是可以放置皇后的位置。

```go
var solutions [][]string

func solveNQueens(n int) [][]string {
    solutions = [][]string{}
    queens := make([]int, n)
    for i := 0; i < n; i++ {
        queens[i] = -1
    }
    columns := map[int]bool{}
    diagonals1, diagonals2 := map[int]bool{}, map[int]bool{}
    backtrack(queens, n, 0, columns, diagonals1, diagonals2)
    return solutions
}

func backtrack(queens []int, n, row int, columns, diagonals1, diagonals2 map[int]bool) {
    if row == n {
        board := generateBoard(queens, n)
        solutions = append(solutions, board)
        return
    }
    // i 表示要在第 i 列上放置皇后。
    for i := 0; i < n; i++ {
        // 如果 i 列或者包含 [row, i] 的斜线上已经放置了皇后，则跳过。
        if columns[i] {
            continue
        }
        diagonal1 := row - i
        if diagonals1[diagonal1] {
            continue
        }
        diagonal2 := row + i
        if diagonals2[diagonal2] {
            continue
        }
        // row 行 i 列上放置一个皇后。
        queens[row] = i
        columns[i] = true
        diagonals1[diagonal1], diagonals2[diagonal2] = true, true
        // 继续在下一行放置。
        backtrack(queens, n, row + 1, columns, diagonals1, diagonals2)
        // 将当前选择消除，继续寻找下一种可能。
        queens[row] = -1
        delete(columns, i)
        delete(diagonals1, diagonal1)
        delete(diagonals2, diagonal2)
    }
}

func generateBoard(queens []int, n int) []string {
    board := []string{}
    for i := 0; i < n; i++ {
        // 第一行用一个字符串表示。
        row := make([]byte, n)
        // 除了第 i 列，其它列都设置成 '.'
        for j := 0; j < n; j++ {
            row[j] = '.'
        }
        row[queens[i]] = 'Q'
        board = append(board, string(row))
    }
    return board
}
```

**复杂度分析**

- 时间复杂度：$O(N!)$，其中 N 是皇后数量。(每选择一行后，剩下的选择都少了至少1种)
- 空间复杂度：$O(N)$，其中 N 是皇后数量。空间复杂度主要取决于递归调用层数、记录每行放置的皇后的列下标的数组以及三个集合，递归调用层数不会超过 N，数组的长度为 N，每个集合的元素个数都不会超过 N。



### 方法二：基于位运算的回溯

方法一使用三个集合记录分别记录每一列以及两个方向的每条斜线上是否有皇后，每个集合最多包含 N 个元素，因此集合的空间复杂度是 $O(N)$。如果利用位运算记录皇后的信息，就可以将记录皇后信息的空间复杂度从 $O(N)$ 降到 $O(1)$。

具体做法是，使用三个整数 $\textit{columns}$、$\textit{diagonals}_1$ 和 $\textit{diagonals}_2$ 分别记录每一列以及两个方向的每条斜线上是否有皇后，每个整数有 N 个二进制位(如果 N 超过了 64 或者 128 呢？)。棋盘的每一列对应每个整数的二进制表示中的一个数位，其中棋盘的最左列对应每个整数的最低二进制位，最右列对应每个整数的最高二进制位。

那么如何根据每次放置的皇后更新三个整数的值呢？在说具体的计算方法之前，首先说一个例子。

棋盘的边长和皇后的数量 N=8。如果棋盘的前两行分别在第 2 列和第 4 列放置了皇后（下标从 0 开始），则棋盘的前两行如下图所示。

![](assets/no_0051_n_queens3.png)

如果要在下一行放置皇后，哪些位置不能放置呢？我们用 0 代表可以放置皇后的位置，1 代表不能放置皇后的位置。

新放置的皇后不能和任何一个已经放置的皇后在同一列，因此不能放置在第 2 列和第 4 列，对应 $\textit{columns}=00010100_{(2)}$。

新放置的皇后不能和任何一个已经放置的皇后在同一条方向一（从左上到右下方向）的斜线上，因此不能放置在第 4 列和第 5 列，对应 $\textit{diagonals}_1=00110000_{(2)}$。其中，第 4 列为其前两行的第 2 列的皇后往右下移动两步的位置，第 5 列为其前一行的第 4 列的皇后往右下移动一步的位置。

新放置的皇后不能和任何一个已经放置的皇后在同一条方向二（从右上到左下方向）的斜线上，因此不能放置在第 0 列和第 3 列，对应 $\textit{diagonals}_2=00001001_{(2)}$。其中，第 0 列为其前两行的第 2 列的皇后往左下移动两步的位置，第 3 列为其前一行的第 4 列的皇后往左下移动一步的位置。

![](assets/no_0051_n_queens4.png)

由此可以得到三个整数的计算方法：

- 初始时，三个整数的值都等于 0，表示没有放置任何皇后；
- 在当前行放置皇后，如果皇后放置在第 i 列，则将三个整数的第 i 个二进制位（指从低到高的第 i 个二进制位）的值设为 1；
- 进入下一行时，$\textit{columns}$ 的值保持不变，$\textit{diagonals}_1$ 左移一位，$\textit{diagonals}_2$ 右移一位(二进制的顺序和图上的棋盘是相反的)，由于棋盘的最左列对应每个整数的最低二进制位，即每个整数的最右二进制位，因此对整数的移位操作方向和对棋盘的移位操作方向相反（对棋盘的移位操作方向是 $\textit{diagonals}_1$ 右移一位，$\textit{diagonals}_2$ 左移一位）。

![](assets/no_0051_n_queens5.png)
![](assets/no_0051_n_queens6.png)
![](assets/no_0051_n_queens7.png)
![](assets/no_0051_n_queens8.png)
![](assets/no_0051_n_queens9.png)
![](assets/no_0051_n_queens10.png)

每次放置皇后时，三个整数的按位或运算的结果即为不能放置皇后的位置，其余位置即为可以放置皇后的位置。可以通过 $(2^n-1)~\&~(\sim(\textit{columns} | \textit{diagonals}_1 | \textit{diagonals}_2))$ 得到可以放置皇后的位置（该结果的值为 1 的位置表示可以放置皇后的位置(是不是因为 0 的话前面可能有无效的位?)），然后遍历这些位置，尝试放置皇后并得到可能的解。

遍历可以放置皇后的位置时，可以利用以下两个按位与运算的性质：

- `x & (−x)` 可以获得 x 的二进制表示中的最低位的 1 的位置；
- `x & (x−1)` 可以将 x 的二进制表示中的最低位的 1 置成 0。

具体做法是，每次获得可以放置皇后的位置中的最低位，并将该位的值置成 0，尝试在该位置放置皇后。这样即可遍历每个可以放置皇后的位置。

```go
var solutions [][]string

func solveNQueens(n int) [][]string {
    solutions = [][]string{}
    queens := make([]int, n)
    for i := 0; i < n; i++ {
        queens[i] = -1
    }
    solve(queens, n, 0, 0, 0, 0)
    return solutions
}

func solve(queens []int, n, row, columns, diagonals1, diagonals2 int) {
    if row == n {
        board := generateBoard(queens, n)
        solutions = append(solutions, board)
        return
    }
    availablePositions := ((1 << n) - 1) & (^(columns | diagonals1 | diagonals2)) // 可选的位置
    for availablePositions != 0 {
        position := availablePositions & (-availablePositions) // 只保留最后一个 1
        availablePositions = availablePositions & (availablePositions - 1) // 将最后一个 1 置 0
        // math/bits. OnesCount 是 1 的个数，position-1后 1 的个数就是 position 中那个 1 的位置。
        column := bits.OnesCount(uint(position - 1))
        queens[row] = column
        solve(queens, n, row + 1, columns | position, (diagonals1 | position) >> 1, (diagonals2 | position) << 1)
        queens[row] = -1
    }
}

func generateBoard(queens []int, n int) []string {
    board := []string{}
    for i := 0; i < n; i++ {
        row := make([]byte, n)
        for j := 0; j < n; j++ {
            row[j] = '.'
        }
        row[queens[i]] = 'Q'
        board = append(board, string(row))
    }
    return board
}
```

**复杂度分析**

- 时间复杂度：$O(N!)$，其中 N 是皇后数量。
- 空间复杂度：$O(N)$，其中 N 是皇后数量。由于使用位运算表示，因此存储皇后信息的空间复杂度是 $O(1)$，空间复杂度主要取决于递归调用层数和记录每行放置的皇后的列下标的数组，递归调用层数不会超过 N，数组的长度为 N。

**小结**

回顾这道题，拿到这道题的时候，其实我们很容易看出需要使用枚举的方法来求解这个问题，当我们不知道用什么办法来枚举是最优的时候，可以从下面三个方向考虑：

- 子集枚举：可以把问题转化成「从 $n^2$ 个格子中选一个子集，使得子集中恰好有 n 个格子，且任意选出两个都不在同行、同列或者同对角线」，这里枚举的规模是 $2^{n^2}$；
- 组合枚举：可以把问题转化成「从 $n^2$ 个格子中选择 n 个，且任意选出两个都不在同行、同列或者同对角线」，这里的枚举规模是 ${n^2} \choose {n}$；
- 排列枚举：因为这里每行只能放置一个皇后，而所有行中皇后的列号正好构成一个 1 到 n 的排列，所以我们可以把问题转化为一个排列枚举，规模是 $n!$。

带入一些 n 进这三种方法验证，就可以知道那种方法的枚举规模是最小的，这里我们发现第三种方法的枚举规模最小。这道题给出的两个方法其实和排列枚举的本质是类似的。
