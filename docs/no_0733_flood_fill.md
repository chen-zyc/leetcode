- [733. 图像渲染](#733-图像渲染)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：广度优先搜索](#方法一广度优先搜索)
    - [方法二：深度优先搜索](#方法二深度优先搜索)

------------------------------

# 733. 图像渲染

## 题目

有一幅以二维整数数组表示的图画，每一个整数表示该图画的像素值大小，数值在 0 到 65535 之间。

给你一个坐标 `(sr, sc)` 表示图像渲染开始的像素值（行 ，列）和一个新的颜色值 newColor，让你重新上色这幅图像。

为了完成上色工作，从初始坐标开始，记录初始坐标的上下左右四个方向上像素值与初始坐标相同的相连像素点，接着再记录这四个方向上符合条件的像素点与他们对应四个方向上像素值与初始坐标相同的相连像素点，……，重复该过程。将所有有记录的像素点的颜色值改为新的颜色值。

最后返回经过上色渲染后的图像。

示例 1:

```
输入: 
image = [[1,1,1],[1,1,0],[1,0,1]]
sr = 1, sc = 1, newColor = 2
输出: [[2,2,2],[2,2,0],[2,0,1]]
解析: 
在图像的正中间，(坐标(sr,sc)=(1,1)),
在路径上所有符合条件的像素点的颜色都被更改成2。
注意，右下角的像素没有更改为2，
因为它不是在上下左右四个方向上与初始点相连的像素点。
```

注意:

- image 和 `image[0]` 的长度在范围 [1, 50] 内。
- 给出的初始点将满足 `0 <= sr < image.length 和 0 <= sc < image[0].length`。
- `image[i][j]` 和 newColor 表示的颜色值在范围 [0, 65535]内。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/flood-fill
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/flood-fill/solution/tu-xiang-xuan-ran-by-leetcode-solution/

**写在前面**

本题要求将给定的二维数组中指定的「色块」染成另一种颜色。「色块」的定义是：直接或间接相邻的同色方格构成的整体。

可以发现，「色块」就是被不同颜色的方格包围的一个同色岛屿。我们从色块中任意一个地方开始，利用广度优先搜索或深度优先搜索即可遍历整个岛屿。

注意：**当目标颜色和初始颜色相同时，我们无需对原数组进行修改**。

### 方法一：广度优先搜索

**思路及算法**

我们从给定的起点开始，进行广度优先搜索。每次搜索到一个方格时，如果其与初始位置的方格颜色相同，就将该方格加入队列，并将该方格的颜色更新，以防止重复入队。

注意：因为初始位置的颜色会被修改，所以我们需要保存初始位置的颜色，以便于之后的更新操作。

```go
var (
    dx = []int{1, 0, 0, -1}
    dy = []int{0, 1, -1, 0}
)

func floodFill(image [][]int, sr int, sc int, newColor int) [][]int {
    currColor := image[sr][sc]
    if currColor == newColor {
        return image
    }
    n, m := len(image), len(image[0])
    queue := [][]int{}
    queue = append(queue, []int{sr, sc})
    image[sr][sc] = newColor
    for i := 0; i < len(queue); i++ {
        cell := queue[i]
        for j := 0; j < 4; j++ {
            mx, my := cell[0] + dx[j], cell[1] + dy[j]
            if mx >= 0 && mx < n && my >= 0 && my < m && image[mx][my] == currColor {
                queue = append(queue, []int{mx, my})
                image[mx][my] = newColor
            }
        }
    }
    return image
}
```

**复杂度分析**

- 时间复杂度：$O(n\times m)$，其中 n 和 m 分别是二维数组的行数和列数。最坏情况下需要遍历所有的方格一次。
- 空间复杂度：$O(n\times m)$，其中 n 和 m 分别是二维数组的行数和列数。主要为队列的开销。



### 方法二：深度优先搜索

**思路及算法**

我们从给定的起点开始，进行深度优先搜索。每次搜索到一个方格时，如果其与初始位置的方格颜色相同，就将该方格的颜色更新，以防止重复搜索；如果不相同，则进行回溯。

注意：因为初始位置的颜色会被修改，所以我们需要保存初始位置的颜色，以便于之后的更新操作。

```go
var (
    dx = []int{1, 0, 0, -1}
    dy = []int{0, 1, -1, 0}
)

func floodFill(image [][]int, sr int, sc int, newColor int) [][]int {
    currColor := image[sr][sc]
    if currColor != newColor {
        dfs(image, sr, sc, currColor, newColor)
    }
    return image
}

func dfs(image [][]int, x, y, color, newColor int) {
    if image[x][y] == color {
        image[x][y] = newColor
        for i := 0; i < 4; i++ {
            mx, my := x + dx[i], y + dy[i]
            if mx >= 0 && mx < len(image) && my >= 0 && my < len(image[0]) {
                dfs(image, mx, my, color, newColor)
            }
        }
    }
}
```

**复杂度分析**

- 时间复杂度：$O(n\times m)$，其中 n 和 m 分别是二维数组的行数和列数。最坏情况下需要遍历所有的方格一次。
- 空间复杂度：$O(n\times m)$，其中 n 和 m 分别是二维数组的行数和列数。主要为栈空间的开销。
