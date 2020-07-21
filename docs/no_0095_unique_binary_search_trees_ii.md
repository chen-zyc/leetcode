- [95. 不同的二叉搜索树 II](#95-不同的二叉搜索树-ii)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：递归](#方法一递归)

------------------------------

# 95. 不同的二叉搜索树 II

## 题目

给定一个整数 n，生成所有由 1 ... n 为节点所组成的 **二叉搜索树** 。

示例：

```
输入：3
输出：
[
  [1,null,3,2],
  [3,2,null,1],
  [3,1,null,null,2],
  [2,1,3],
  [1,null,2,null,3]
]
解释：
以上的输出对应以下 5 种不同结构的二叉搜索树：

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
```

提示： `0 <= n <= 8`

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/unique-binary-search-trees-ii
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

## 题解

> 思路想出来了，但是程序没有写出来😥

--------------------

> 链接：https://leetcode-cn.com/problems/unique-binary-search-trees-ii/solution/bu-tong-de-er-cha-sou-suo-shu-ii-by-leetcode-solut/

### 方法一：递归

**思路与算法**

二叉搜索树关键的性质是**根节点的值大于左子树所有节点的值，小于右子树所有节点的值**，且左子树和右子树也同样为二叉搜索树。因此在生成所有可行的二叉搜索树的时候，假设当前序列长度为 n，如果我们枚举根节点的值为 i，那么根据二叉搜索树的性质我们可以知道左子树的节点值的集合为 $[1 \ldots i-1]$，右子树的节点值的集合为 $[i+1 \ldots n]$。而左子树和右子树的生成相较于原问题是一个序列长度缩小的子问题，因此我们可以想到用**递归**的方法来解决这道题目。

我们定义 `generateTrees(start, end)` 函数表示当前值的集合为 `[start,end]`，返回序列 `[start,end]` 生成的所有可行的二叉搜索树。按照上文的思路，我们考虑枚举 `[start,end]` 中的值 `i` 为当前二叉搜索树的根，那么序列划分为了 `[start,i−1]` 和 `[i+1,end]` 两部分。我们递归调用这两部分，即 `generateTrees(start, i - 1)` 和 `generateTrees(i + 1, end)`，获得所有可行的左子树和可行的右子树，那么最后一步我们只要从可行左子树集合中选一棵，再从可行右子树集合中选一棵拼接到根节点上，并将生成的二叉搜索树放入答案数组即可。

递归的入口即为 `generateTrees(1, n)`，出口为当 `start>end` 的时候，当前二叉搜索树为空，返回空节点即可。

```go
func generateTrees(n int) []*TreeNode {
    if n == 0 {
        return nil
    }
    return helper(1, n)
}

func helper(start, end int) []*TreeNode {
    if start > end {
        // 这是这个地方没有想出来。这里如果直接返回的是 nil 的话（空数组），那么在下面遍历左右子树的时候可能左子树就没有，同样右子树也可能没有。
        return []*TreeNode{nil}
    }
    allTrees := []*TreeNode{}
    // 枚举可行根节点
    for i := start; i <= end; i++ {
        // 获得所有可行的左子树集合
        leftTrees := helper(start, i - 1)
        // 获得所有可行的右子树集合
        rightTrees := helper(i + 1, end)
        // 从左子树集合中选出一棵左子树，从右子树集合中选出一棵右子树，拼接到根节点上
        for _, left := range leftTrees {
            for _, right := range rightTrees {
                currTree := &TreeNode{i, nil, nil}
                currTree.Left = left
                currTree.Right = right
                allTrees = append(allTrees, currTree)
            }
        }
    }
    return allTrees
}
```

**复杂度分析**

- 时间复杂度 : 整个算法的时间复杂度取决于「可行二叉搜索树的个数」，而对于 n 个点生成的二叉搜索树数量等价于数学上第 n 个「卡特兰数」，用 $G_n$ 表示。卡特兰数具体的细节请读者自行查询，这里不再赘述，只给出结论。生成一棵二叉搜索树需要 $O(n)$ 的时间复杂度，一共有 $G_n$ 棵二叉搜索树，也就是 $O(nG_n)$。而卡特兰数以 $\frac{4^n}{n^{3/2}}$ 增长，因此总时间复杂度为 $O(\frac{4^n}{n^{1/2}})$。
- 空间复杂度 : n 个点生成的二叉搜索树有 $G_n$ 棵，每棵有 n 个节点，因此存储的空间需要 $O(nG_n) = O(\frac{4^n}{n^{1/2}})$，递归函数需要 $O(n)$ 的栈空间，因此总空间复杂度为 $O(\frac{4^n}{n^{1/2}})$。 