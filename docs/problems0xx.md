- [84. 柱状图中最大的矩形](#84-柱状图中最大的矩形)
- [85. 最大矩形](#85-最大矩形)

------------------------------

# 84. 柱状图中最大的矩形

给定 n 个非负整数，用来表示柱状图中各个柱子的高度。每个柱子彼此相邻，且宽度为 1 。

求在该柱状图中，能够勾勒出来的矩形的最大面积。

![](assets/0084_largest-rectangle-in-histogram1.png)

以上是柱状图的示例，其中每个柱子的宽度为 1，给定的高度为 `[2,1,5,6,2,3]`。

![](assets/0084_largest-rectangle-in-histogram2.png)

图中阴影部分为所能勾勒出的最大矩形面积，其面积为 10 个单位。

链接：https://leetcode-cn.com/problems/largest-rectangle-in-histogram/

**官方题解**

链接：https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/zhu-zhuang-tu-zhong-zui-da-de-ju-xing-by-leetcode-/

官方题解太难懂了，不过视频讲解的还可以，建议看下。

![](assets/0084_largest-rectangle-in-histogram5.png)

```go
func largestRectangleArea(heights []int) int {
    n := len(heights)
    left, right := make([]int, n), make([]int, n)
    mono_stack := []int{}
    for i := 0; i < n; i++ {
        // 把所有 >= height[i] 的都弹走。
        for len(mono_stack) > 0 && heights[mono_stack[len(mono_stack)-1]] >= heights[i] {
            mono_stack = mono_stack[:len(mono_stack)-1]
        }
        if len(mono_stack) == 0 {
            left[i] = -1
        } else {
            // left[i] 记录的是 i 左侧最近的比 height[i] 小的下标。
            left[i] = mono_stack[len(mono_stack)-1]
        }
        mono_stack = append(mono_stack, i)
    }
    mono_stack = []int{}
    for i := n - 1; i >= 0; i-- {
        for len(mono_stack) > 0 && heights[mono_stack[len(mono_stack)-1]] >= heights[i] {
            mono_stack = mono_stack[:len(mono_stack)-1]
        }
        if len(mono_stack) == 0 {
            right[i] = n
        } else {
            right[i] = mono_stack[len(mono_stack)-1]
        }
        mono_stack = append(mono_stack, i)
    }
    ans := 0
    for i := 0; i < n; i++ {
        // left 和 right 之间的，不包括 left 和 right。
        ans = max(ans, (right[i] - left[i] - 1) * heights[i])
    }
    return ans
}

func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}
```

复杂度分析

- 时间复杂度：$O(N)$。
- 空间复杂度：$O(N)$。

**方法二：单调栈 + 常数优化**

在方法一中，我们首先从左往右对数组进行遍历，借助单调栈求出了每根柱子的左边界，随后从右往左对数组进行遍历，借助单调栈求出了每根柱子的右边界。那么我们是否可以只遍历一次就求出答案呢？

答案是可以的。在方法一中，我们在对位置 i 进行入栈操作时，确定了它的左边界。从直觉上来说，与之对应的我们在对位置 i 进行**出栈操作时可以确定它的右边界**！仔细想一想，这确实是对的。当位置 i 被弹出栈时，说明此时遍历到的位置 $i_0$​ 的高度小于等于 $\textit{height}[i]$，并且在 $i_0$​ 与 i 之间没有其他高度小于等于 $\textit{height}[i]$ 的柱子。这是因为，如果在 i 和 $i_0$​ 之间还有其它位置的高度小于等于 $\textit{height}[i]$ 的，那么在遍历到那个位置的时候，i 应该已经被弹出栈了。所以位置 $i_0$​ 就是位置 i 的右边界。

等等，我们需要的是「一根柱子的左侧且最近的小于其高度的柱子」，但这里我们求的是小于等于，那么会造成什么影响呢？答案是：我们确实无法求出正确的右边界，但对最终的答案没有任何影响。这是因为在答案对应的矩形中，如果有若干个柱子的高度都等于矩形的高度，那么最右侧的那根柱子是可以求出正确的右边界的，而我们没有对求出左边界的算法进行任何改动，因此最终的答案还是可以从最右侧的与矩形高度相同的柱子求得的。读者可以仔细思考一下这一步。

在遍历结束后，栈中仍然有一些位置，这些位置对应的右边界就是位置为 n 的「哨兵」。我们可以将它们依次出栈并更新右边界，也可以在初始化右边界数组时就将所有的元素的值置为 n。

```go
func largestRectangleArea(heights []int) int {
    n := len(heights)
    left, right := make([]int, n), make([]int, n)
    for i := 0; i < n; i++ {
        right[i] = n
    }
    mono_stack := []int{}
    for i := 0; i < n; i++ {
        for len(mono_stack) > 0 && heights[mono_stack[len(mono_stack)-1]] >= heights[i] {
            // 找到比 i 高的，它的右边界就是当前下标，然后把它弹出去。
            // 那弹出去的那个下标的左边界是谁呢？应该是前面那个下标，如果它和前面那个下标的高度相同呢？
            right[mono_stack[len(mono_stack)-1]] = i
            mono_stack = mono_stack[:len(mono_stack)-1]
        }
        if len(mono_stack) == 0 {
            left[i] = -1
        } else {
            left[i] = mono_stack[len(mono_stack)-1]
        }
        mono_stack = append(mono_stack, i)
    }
    ans := 0
    for i := 0; i < n; i++ {
        ans = max(ans, (right[i] - left[i] - 1) * heights[i])
    }
    return ans
}

func max(x, y int) int {
    if x > y {
        return x
    }
    return y
}
```

复杂度分析

- 时间复杂度：$O(N)$。
- 空间复杂度：$O(N)$。



------------------------------

下面的截图是截取的[这个题解](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/bao-li-jie-fa-zhan-by-liweiwei1419/)的。

![](assets/0084_largest-rectangle-in-histogram3.png)
![](assets/0084_largest-rectangle-in-histogram4.png)

> 上面截图看不清楚的内容是：这是因为我们就是想确定 6 的宽度，6 的宽度确定完了，其实我们就不需要它了，这个 5 的高度和这个 5 的

```java
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {

    public int largestRectangleArea(int[] heights) {
        int len = heights.length;
        if (len == 0) {
            return 0;
        }
        if (len == 1) {
            return heights[0];
        }

        int res = 0;
        Deque<Integer> stack = new ArrayDeque<>(len);
        for (int i = 0; i < len; i++) {
            // 这个 while 很关键，因为有可能不止一个柱形的最大宽度可以被计算出来
            while (!stack.isEmpty() && heights[i] < heights[stack.peekLast()]) { // 栈顶比当前元素的高度大。
                int curHeight = heights[stack.pollLast()];
                // 如果是相同的，那么也弹出去，这样计算宽度的时候就不会出错了。
                while (!stack.isEmpty() && heights[stack.peekLast()] == curHeight) {
                    stack.pollLast();
                }

                int curWidth;
                if (stack.isEmpty()) {
                    curWidth = i;
                } else {
                    // 因为是递增的（不是单调递增），所以它前面的比它矮。
                    curWidth = i - stack.peekLast() - 1;
                }

                res = Math.max(res, curHeight * curWidth);
            }
            stack.addLast(i);
        }

        while (!stack.isEmpty()) {
            int curHeight = heights[stack.pollLast()];
            while (!stack.isEmpty() && heights[stack.peekLast()] == curHeight) {
                stack.pollLast();
            }
            int curWidth;
            if (stack.isEmpty()) {
                curWidth = len;
            } else {
                curWidth = len - stack.peekLast() - 1;
            }
            res = Math.max(res, curHeight * curWidth);
        }
        return res;
    }
}
```

下面的是加入了哨兵的版本：

```java
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {

    public int largestRectangleArea(int[] heights) {
        int len = heights.length;
        if (len == 0) {
            return 0;
        }

        if (len == 1) {
            return heights[0];
        }

        int res = 0;

        // 在前后都加入一个高度为 0 的哨兵。
        int[] newHeights = new int[len + 2];
        newHeights[0] = 0;
        System.arraycopy(heights, 0, newHeights, 1, len);
        newHeights[len + 1] = 0;
        len += 2;
        heights = newHeights;

        Deque<Integer> stack = new ArrayDeque<>(len);
        // 先放入哨兵，在循环里就不用做非空判断
        stack.addLast(0);
        
        for (int i = 1; i < len; i++) { // 这里从 1 开始的，是不是就不用在 heights 前面加哨兵了？
            while (heights[i] < heights[stack.peekLast()]) {
                // 这里没有处理两个下标的高度相同的情况。
                int curHeight = heights[stack.pollLast()];
                int curWidth = i - stack.peekLast() - 1;
                res = Math.max(res, curHeight * curWidth);
            }
            stack.addLast(i);
        }
        return res;
    }

    public static void main(String[] args) {
        // int[] heights = {2, 1, 5, 6, 2, 3};
        int[] heights = {1, 1};
        Solution solution = new Solution();
        int res = solution.largestRectangleArea(heights);
        System.out.println(res);
    }
}
```

> 为什么可以不需要处理相同高度的情况？
>
> 如果两个下标 i 和 j 对应的高度是一样的，那么它们最后计算出来的面积应该是一样大的（左边界一样，右边界也一样）。所以计算 j 的时候计算对了就可以了。



# 85. 最大矩形

给定一个仅包含 0 和 1 、大小为 rows x cols 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。

示例 1：

![](assets/0085_maximal-rectangle1.jpg)

```
输入：matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
输出：6
解释：最大矩形如上图所示。
```

示例 2：

```
输入：matrix = []
输出：0
```

示例 3：

```
输入：matrix = [["0"]]
输出：0
```

示例 4：

```
输入：matrix = [["1"]]
输出：1
```

示例 5：

```
输入：matrix = [["0","0"]]
输出：0
```

提示：

- rows == matrix.length
- cols == `matrix[0].length`
- 0 <= row, cols <= 200
- `matrix[i][j]` 为 '0' 或 '1'

链接：https://leetcode-cn.com/problems/maximal-rectangle

**官方题解**

![](assets/0085_maximal-rectangle2.png)

```go
func maximalRectangle(matrix [][]byte) (ans int) {
    if len(matrix) == 0 {
        return
    }
    m, n := len(matrix), len(matrix[0])
    // 每个点连续 1 的个数。
    left := make([][]int, m)
    for i, row := range matrix {
        left[i] = make([]int, n)
        for j, v := range row {
            if v == '0' {
                continue
            }
            if j == 0 {
                left[i][j] = 1
            } else {
                left[i][j] = left[i][j-1] + 1
            }
        }
    }
    for i, row := range matrix {
        for j, v := range row {
            if v == '0' {
                continue
            }
            // 以 [i][j] 为右下角时矩阵的面积。
            width := left[i][j]
            area := width
            for k := i - 1; k >= 0; k-- {
                width = min(width, left[k][j])
                area = max(area, (i-k+1)*width)
            }
            ans = max(ans, area)
        }
    }
    return
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
```

复杂度分析

- 时间复杂度：$O(m^2n)$，其中 m 和 n 分别是矩阵的行数和列数。计算 left 矩阵需要 $O(mn)$ 的时间。随后对于矩阵的每个点，需要 $O(m)$ 的时间枚举高度。故总的时间复杂度为 $O(mn) + O(mn) \cdot O(m) = O(m^2n)$。
- 空间复杂度：$O(mn)$，其中 m 和 n 分别是矩阵的行数和列数。我们分配了一个与给定矩阵等大的数组，用于存储每个元素的左边连续 1 的数量。

方法二：单调栈

思路与算法

在方法一中，我们讨论了将输入拆分成一系列的柱状图。为了计算矩形的最大面积，我们只需要计算每个柱状图中的最大面积，并找到全局最大值。

我们可以使用[「84. 柱状图中最大的矩形的官方题解」](https://leetcode-cn.com/problems/largest-rectangle-in-histogram/solution/zhu-zhuang-tu-zhong-zui-da-de-ju-xing-by-leetcode-/)中的单调栈的做法，将其应用在我们生成的柱状图中。

```go
func maximalRectangle(matrix [][]byte) (ans int) {
    if len(matrix) == 0 {
        return
    }
    m, n := len(matrix), len(matrix[0])
    // 每个点连续 1 的数量。
    left := make([][]int, m)
    for i, row := range matrix {
        left[i] = make([]int, n)
        for j, v := range row {
            if v == '0' {
                continue
            }
            if j == 0 {
                left[i][j] = 1
            } else {
                left[i][j] = left[i][j-1] + 1
            }
        }
    }
    // 像是倒了的柱状图，第 j 列是 x 轴。
    for j := 0; j < n; j++ { // 对于每一列，使用基于柱状图的方法
        up := make([]int, m)
        down := make([]int, m)
        stk := []int{}
        for i, l := range left {
            for len(stk) > 0 && left[stk[len(stk)-1]][j] >= l[j] {
                stk = stk[:len(stk)-1]
            }
            up[i] = -1
            if len(stk) > 0 {
                up[i] = stk[len(stk)-1]
            }
            stk = append(stk, i)
        }
        stk = nil
        for i := m - 1; i >= 0; i-- {
            for len(stk) > 0 && left[stk[len(stk)-1]][j] >= left[i][j] {
                stk = stk[:len(stk)-1]
            }
            down[i] = m
            if len(stk) > 0 {
                down[i] = stk[len(stk)-1]
            }
            stk = append(stk, i)
        }
        for i, l := range left {
            height := down[i] - up[i] - 1
            area := height * l[j]
            ans = max(ans, area)
        }
    }
    return
}

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}
```

读者可以自行比对上面的代码与此前第 84 题的代码的相似之处。

复杂度分析

- 时间复杂度：$O(mn)$，其中 m 和 n 分别是矩阵的行数和列数。计算 left 矩阵需要 $O(mn)$ 的时间；对每一列应用柱状图算法需要 $O(m)$ 的时间，一共需要 $O(mn)$ 的时间。
- 空间复杂度：$O(mn)$，其中 m 和 n 分别是矩阵的行数和列数。我们分配了一个与给定矩阵等大的数组，用于存储每个元素的左边连续 1 的数量。

链接：https://leetcode-cn.com/problems/maximal-rectangle/solution/zui-da-ju-xing-by-leetcode-solution-bjlu/