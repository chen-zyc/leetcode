- [841. 钥匙和房间](#841-钥匙和房间)
  - [官方题解](#官方题解)
    - [方法一：深度优先搜索](#方法一深度优先搜索)
    - [方法二：广度优先搜索](#方法二广度优先搜索)

------------------------------

# 841. 钥匙和房间

有 N 个房间，开始时你位于 0 号房间。每个房间有不同的号码：`0，1，2，...，N-1`，并且房间里可能有一些钥匙能使你进入下一个房间。

在形式上，对于每个房间 `i` 都有一个钥匙列表 `rooms[i]`，每个钥匙 `rooms[i][j]` 由 `[0,1，...，N-1]` 中的一个整数表示，其中 `N = rooms.length`。 钥匙 `rooms[i][j] = v` 可以打开编号为 `v` 的房间。

最初，除 0 号房间外的其余所有房间都被锁住。

你可以自由地在房间之间来回走动。

如果能进入每个房间返回 true，否则返回 false。

> 钥匙能重复使用吗？房间开了之后以后就能直接进去了吗？
> 看了题解之后才发现想多了。一旦房间被打开，就可以随意进出了。

示例 1：

```
输入: [[1],[2],[3],[]]
输出: true
解释:  
我们从 0 号房间开始，拿到钥匙 1。
之后我们去 1 号房间，拿到钥匙 2。
然后我们去 2 号房间，拿到钥匙 3。
最后我们去了 3 号房间。
由于我们能够进入每个房间，我们返回 true。
```

示例 2：

```
输入：[[1,3],[3,0,1],[2],[0]]
输出：false
解释：我们不能进入 2 号房间。
```

提示：

- `1 <= rooms.length <= 1000`
- `0 <= rooms[i].length <= 1000`
- 所有房间中的钥匙数量总计不超过 3000。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/keys-and-rooms
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/keys-and-rooms/solution/yao-chi-he-fang-jian-by-leetcode-solution/

**前言**

当 $x$ 号房间中有 $y$ 号房间的钥匙时，我们就可以从 $x$ 号房间去往 $y$ 号房间。如果我们将这 $n$ 个房间看成有向图中的 $n$ 个节点，那么上述关系就可以看作是图中的 $x$ 号点到 $y$ 号点的一条有向边。

这样一来，问题就变成了给定一张有向图，询问从 $0$ 号节点出发是否能够到达所有的节点。

### 方法一：深度优先搜索

**思路及解法**

我们可以使用深度优先搜索的方式遍历整张图，统计可以到达的节点个数，并利用数组 $\textit{vis}$ 标记当前节点是否访问过，以防止重复访问。

```go
var (
    num int
    vis []bool
)

func canVisitAllRooms(rooms [][]int) bool {
    n := len(rooms)
    num = 0
    vis = make([]bool, n)
    dfs(rooms, 0)
    return num == n
}

func dfs(rooms [][]int, x int) {
    vis[x] = true
    num++
    for _, it := range rooms[x] {
        if !vis[it] {
            dfs(rooms, it)
        }
    }
}
```

**复杂度分析**

- 时间复杂度：$O(n+m)$，其中 n 是房间的数量，m 是所有房间中的钥匙数量的总数。
- 空间复杂度：$O(n)$，其中 n 是房间的数量。主要为栈空间的开销。


### 方法二：广度优先搜索

**思路及解法**

我们也可以使用广度优先搜索的方式遍历整张图，统计可以到达的节点个数，并利用数组 $\textit{vis}$ 标记当前节点是否访问过，以防止重复访问。

```go
func canVisitAllRooms(rooms [][]int) bool {
    n := len(rooms)
    num := 0
    vis := make([]bool, n)
    queue := []int{}
    vis[0] = true
    queue = append(queue, 0)
    for i := 0; i < len(queue); i++ {
        x := queue[i]
        num++
        for _, it := range rooms[x] {
            if !vis[it] {
                vis[it] = true
                queue = append(queue, it)
            }
        }
    }
    return num == n
}
```

**复杂度分析**

- 时间复杂度：$O(n+m)$，其中 n 是房间的数量，m 是所有房间中的钥匙数量的总数。
- 空间复杂度：$O(n)$，其中 n 是房间的数量。主要为队列的开销。
