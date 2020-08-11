- [130. 被围绕的区域](#130-被围绕的区域)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：深度优先搜索](#方法一深度优先搜索)
    - [方法二：广度优先搜索](#方法二广度优先搜索)

------------------------------

# 130. 被围绕的区域

## 题目

给定一个二维的矩阵，包含 'X' 和 'O'（字母 O）。

找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X' 填充。

示例:

```
X X X X
X O O X
X X O X
X O X X
```

运行你的函数后，矩阵变为：

```
X X X X
X X X X
X X X X
X O X X
```

解释:

被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 任何不在边界上，或不与边界上的 'O' 相连的 'O' 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/surrounded-regions
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/surrounded-regions/solution/bei-wei-rao-de-qu-yu-by-leetcode-solution/

**写在前面**

本题给定的矩阵中有三种元素：

- 字母 `X`；
- 被字母 `X` 包围的字母 `O`；
- 没有被字母 `X` 包围的字母 `O`。

本题要求将所有被字母 `X` 包围的字母 `O` 都变为字母 `X` ，但很难判断哪些 `O` 是被包围的，哪些 `O` 不是被包围的。

注意到题目解释中提到：**任何边界上的 `O` 都不会被填充为 `X`**。 我们可以想到，所有的不被包围的 `O` 都直接或间接与边界上的 `O` 相连。我们可以利用这个性质判断 `O` 是否在边界上，具体地说：

- 对于每一个边界上的 `O`，我们以它为起点，标记所有与它直接或间接相连的字母 `O`；
- 最后我们遍历这个矩阵，对于每一个字母：
    - 如果该字母被标记过，则该字母为没有被字母 `X` 包围的字母 `O`，我们将其还原为字母 `O`；
    - 如果该字母没有被标记过，则该字母为被字母 `X` 包围的字母 `O`，我们将其修改为字母 `X`。

> 逆向思维 😳

### 方法一：深度优先搜索

**思路及解法**

我们可以使用深度优先搜索实现标记操作。在下面的代码中，我们把标记过的字母 `O` 修改为字母 `A`。

```go
var n, m int

func solve(board [][]byte)  {
    if len(board) == 0 || len(board[0]) == 0 {
        return
    }
    n, m = len(board), len(board[0])
    for i := 0; i < n; i++ {
        dfs(board, i, 0) // 左边界
        dfs(board, i, m - 1) // 右边界
    }
    for i := 1; i < m - 1; i++ { // [1, m-1)
        dfs(board, 0, i) // 上边界
        dfs(board, n - 1, i) // 下边界
    }
    for i := 0; i < n; i++ {
        for j := 0; j < m; j++ {
            if board[i][j] == 'A' { // 与边界相连，没有被包围，还原回来。
                board[i][j] = 'O'
            } else if board[i][j] == 'O' { // 没有与边界相连，被包围了。
                board[i][j] = 'X'
            }
        }
    }
}

// 如果 [x][y] 是 'O'，那么标记成 'A'，并且继续把四个方向相邻的位置检查一遍。
func dfs(board [][]byte, x, y int) {
    if x < 0 || x >= n || y < 0 || y >= m || board[x][y] != 'O' {
        return
    }
    board[x][y] = 'A'
    dfs(board, x + 1, y)
    dfs(board, x - 1, y)
    dfs(board, x, y + 1)
    dfs(board, x, y - 1)
}
```

**复杂度分析**

- 时间复杂度：$O(n \times m)$，其中 n 和 m 分别为矩阵的行数和列数。深度优先搜索过程中，每一个点至多只会被标记一次。
- 空间复杂度：$O(n \times m)$，其中 n 和 m 分别为矩阵的行数和列数。主要为深度优先搜索的栈的开销。

### 方法二：广度优先搜索

**思路及解法**

我们可以使用广度优先搜索实现标记操作。在下面的代码中，我们把标记过的字母 `O` 修改为字母 `A`。

```go
var (
    dx = [4]int{1, -1, 0, 0}
    dy = [4]int{0, 0, 1, -1}
)
func solve(board [][]byte)  {
    if len(board) == 0 || len(board[0]) == 0 {
        return
    }
    n, m := len(board), len(board[0])
    queue := [][]int{}
    // 左右边界的 'O' 加入到队列中。
    for i := 0; i < n; i++ {
        if board[i][0] == 'O' {
            queue = append(queue, []int{i, 0})
        }
        if board[i][m-1] == 'O' {
            queue = append(queue, []int{i, m - 1})
        }
    }
    // 上下边界的 'O' 加入到队列中。
    for i := 1; i < m - 1; i++ {
        if board[0][i] == 'O' {
            queue = append(queue, []int{0, i})
        }
        if board[n-1][i] == 'O' {
            queue = append(queue, []int{n - 1, i})
        }
    }
    for len(queue) > 0 {
        cell := queue[0]
        queue = queue[1:]
        x, y := cell[0], cell[1]
        // 标记成 'A'，如果相连的点也是 'O'，继续加入到队列中。
        board[x][y] = 'A'
        for i := 0; i < 4; i++ {
            mx, my := x + dx[i], y + dy[i]
            if mx < 0 || my < 0 || mx >= n || my >= m || board[mx][my] != 'O' {
                continue
            }
            queue = append(queue, []int{mx, my})
        }
    }
    for i := 0; i < n; i++ {
        for j := 0; j < m; j++ {
            if board[i][j] == 'A' {
                board[i][j] = 'O'
            } else if board[i][j] == 'O' {
                board[i][j] = 'X'
            }
        }
    }
}
```

**复杂度分析**

- 时间复杂度：$O(n \times m)$，其中 n 和 m 分别为矩阵的行数和列数。广度优先搜索过程中，每一个点至多只会被标记一次。
- 空间复杂度：$O(n \times m)$，其中 n 和 m 分别为矩阵的行数和列数。主要为广度优先搜索的队列的开销。

