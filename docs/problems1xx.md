- [101. 对称二叉树](#101-对称二叉树)
- [153. 寻找旋转排序数组中的最小值](#153-寻找旋转排序数组中的最小值)
- [154. 寻找旋转排序数组中的最小值 II](#154-寻找旋转排序数组中的最小值-ii)

------------------------------

# 101. 对称二叉树

给你一个二叉树的根节点 root ， 检查它是否轴对称。

```
示例 1：
输入：root = [1,2,2,3,4,4,3]
输出：true

示例 2：
输入：root = [1,2,2,null,3,null,3]
输出：false
```

提示：

- 树中节点数目在范围 [1, 1000] 内
- -100 <= Node.val <= 100

进阶：你可以运用递归和迭代两种方法解决这个问题吗？

链接：https://leetcode-cn.com/problems/symmetric-tree

**官方题解**

**方法一：递归**

如果一个树的左子树与右子树镜像对称，那么这个树是对称的。

![fig1](problems1xx.assets/101_fig1.PNG)

因此，该问题可以转化为：两个树在什么情况下互为镜像？

如果同时满足下面的条件，两个树互为镜像：

- 它们的两个根结点具有相同的值
- 每个树的右子树都与另一个树的左子树镜像对称

我们可以实现这样一个递归函数，通过「同步移动」两个指针的方法来遍历这棵树，p 指针和 q 指针一开始都指向这棵树的根，随后 p 右移时，q 左移，p 左移时，q 右移。每次检查当前 p 和 q 节点的值是否相等，如果相等再判断左右子树是否对称。

```go
func isSymmetric(root *TreeNode) bool {
    return check(root, root)
}

func check(p, q *TreeNode) bool {
    if p == nil && q == nil {
        return true
    }
    if p == nil || q == nil {
        return false
    }
    return p.Val == q.Val && check(p.Left, q.Right) && check(p.Right, q.Left) 
}
```

复杂度分析

假设树上一共 $n$ 个节点。

- 时间复杂度：这里遍历了这棵树，渐进时间复杂度为 $O(n)$。
- 空间复杂度：这里的空间复杂度和递归使用的栈空间有关，这里递归层数不超过 $n$，故渐进空间复杂度为 $O(n)$。

**方法二：迭代**

「方法一」中我们用递归的方法实现了对称性的判断，那么如何用迭代的方法实现呢？首先我们引入一个队列，这是把递归程序改写成迭代程序的常用方法。初始化时我们把根节点入队两次。每次提取两个结点并比较它们的值（队列中每两个连续的结点应该是相等的，而且它们的子树互为镜像），然后将两个结点的左右子结点按相反的顺序插入队列中。当队列为空时，或者我们检测到树不对称（即从队列中取出两个不相等的连续结点）时，该算法结束。

```go
func isSymmetric(root *TreeNode) bool {
    u, v := root, root
    q := []*TreeNode{}
    q = append(q, u)
    q = append(q, v)
    for len(q) > 0 {
        u, v = q[0], q[1]
        q = q[2:]
        if u == nil && v == nil {
            continue
        }
        if u == nil || v == nil {
            return false
        }
        if u.Val != v.Val {
            return false
        }
        q = append(q, u.Left)
        q = append(q, v.Right)

        q = append(q, u.Right)
        q = append(q, v.Left)
    }
    return true
}
```

复杂度分析

- 时间复杂度：$O(n)$，同「方法一」。
- 空间复杂度：这里需要用一个队列来维护节点，每个节点最多进队一次，出队一次，队列中最多不会超过 $n$ 个点，故渐进空间复杂度为 $O(n)$。




# 153. 寻找旋转排序数组中的最小值

已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 **旋转** 后，得到输入数组。例如，原数组 `nums = [0,1,2,4,5,6,7]` 在变化后可能得到：

- 若旋转 4 次，则可以得到 `[4,5,6,7,0,1,2]`
- 若旋转 4 次，则可以得到 `[0,1,2,4,5,6,7]`（这应该是旋转了 7 次吧？）

注意，数组 `[a[0], a[1], a[2], ..., a[n-1]]` **旋转一次** 的结果为数组 `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]` 。

给你一个元素值 **互不相同** 的数组 nums ，它原来是一个升序排列的数组，并按上述情形进行了多次旋转。请你找出并返回数组中的 **最小元素** 。


示例 1：

```
输入：nums = [3,4,5,1,2]
输出：1
解释：原数组为 [1,2,3,4,5] ，旋转 3 次得到输入数组。
```

示例 2：

```
输入：nums = [4,5,6,7,0,1,2]
输出：0
解释：原数组为 [0,1,2,4,5,6,7] ，旋转 4 次得到输入数组。
```

示例 3：

```
输入：nums = [11,13,15,17]
输出：11
解释：原数组为 [11,13,15,17] ，旋转 4 次得到输入数组。
```

提示：

- n == nums.length
- 1 <= n <= 5000
- -5000 <= `nums[i]` <= 5000
- nums 中的所有整数 互不相同
- nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转

链接：https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array


**官方题解**

方法一：二分查找

一个不包含重复元素的升序数组在经过旋转之后，可以得到下面可视化的折线图：

![](assets/0153_find-minimum-in-rotated-sorted-array1.png)

其中横轴表示数组元素的下标，纵轴表示数组元素的值。图中标出了最小值的位置，是我们需要查找的目标。

我们考虑数组中的最后一个元素 $x$：在最小值右侧的元素（不包括最后一个元素本身），它们的值一定都严格小于 $x$；而在最小值左侧的元素，它们的值一定都严格大于 $x$。因此，我们可以根据这一条性质，通过二分查找的方法找出最小值。

在二分查找的每一步中，左边界为 `low`，右边界为 `high`，区间的中点为 `pivot`，最小值就在该区间内。我们将中轴元素 `nums[pivot]` 与右边界元素 `nums[high]` 进行比较，可能会有以下的三种情况：

第一种情况是 `nums[pivot]<nums[high]`。如下图所示，这说明 `nums[pivot]` 是最小值右侧的元素，因此我们可以忽略二分查找区间的右半部分。

![](assets/0153_find-minimum-in-rotated-sorted-array2.png)

第二种情况是 `nums[pivot]>nums[high]`。如下图所示，这说明 `nums[pivot]` 是最小值左侧的元素，因此我们可以忽略二分查找区间的左半部分。

![](assets/0153_find-minimum-in-rotated-sorted-array3.png)

由于数组不包含重复元素，并且只要当前的区间长度不为 1，`pivot` 就不会与 `high` 重合；而如果当前的区间长度为 1，这说明我们已经可以结束二分查找了。因此不会存在 `nums[pivot]=nums[high]` 的情况。

当二分查找结束时，我们就得到了最小值所在的位置。

```go
func findMin(nums []int) int {
    low, high := 0, len(nums) - 1
    for low < high {
        pivot := low + (high - low) / 2
        if nums[pivot] < nums[high] {
            high = pivot
        } else {
            low = pivot + 1
        }
    }
    return nums[low]
}
```

- 时间复杂度：时间复杂度为 $O(\log n)$，其中 n 是数组 `nums` 的长度。在二分查找的过程中，每一步会忽略一半的区间。
- 空间复杂度：$O(1)$。

链接：https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/solution/xun-zhao-xuan-zhuan-pai-xu-shu-zu-zhong-5irwp/

**我的图解**

![](assets/0153_find-minimum-in-rotated-sorted-array4.png)


# 154. 寻找旋转排序数组中的最小值 II

已知一个长度为 n 的数组，预先按照升序排列，经由 1 到 n 次 旋转 后，得到输入数组。例如，原数组 `nums = [0,1,4,4,5,6,7]` 在变化后可能得到：

- 若旋转 4 次，则可以得到 `[4,5,6,7,0,1,4]`
- 若旋转 7 次，则可以得到 `[0,1,4,4,5,6,7]`

注意，数组 `[a[0], a[1], a[2], ..., a[n-1]]` 旋转一次 的结果为数组 `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]`。

给你一个可能存在 **重复** 元素值的数组 nums ，它原来是一个升序排列的数组，并按上述情形进行了多次旋转。请你找出并返回数组中的 最小元素 。

示例 1：

```
输入：nums = [1,3,5]
输出：1
```

示例 2：

```
输入：nums = [2,2,2,0,1]
输出：0
```

提示：

- n == nums.length
- 1 <= n <= 5000
- -5000 <= `nums[i]` <= 5000
- nums 原来是一个升序排序的数组，并进行了 1 至 n 次旋转


进阶：

- 这道题是 寻找旋转排序数组中的最小值 的延伸题目。
- 允许重复会影响算法的时间复杂度吗？会如何影响，为什么？

链接：https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array-ii

**官方题解**

本题是「153. 寻找旋转排序数组中的最小值」的延伸。读者可以先尝试第 153 题，体会在旋转数组中进行二分查找的思路，再来尝试解决本题。

**方法一：二分查找**

一个包含重复元素的升序数组在经过旋转之后，可以得到下面可视化的折线图：

![](assets/0154_find-minimum-in-rotated-sorted-array-ii1.png)

其中横轴表示数组元素的下标，纵轴表示数组元素的值。图中标出了最小值的位置，是我们需要查找的目标。

我们考虑数组中的最后一个元素 $x$：在最小值右侧的元素，它们的值一定都小于等于 $x$；而在最小值左侧的元素，它们的值一定都大于等于 $x$。因此，我们可以根据这一条性质，通过二分查找的方法找出最小值。

在二分查找的每一步中，左边界为 `low`，右边界为 `high`，区间的中点为 `pivot`，最小值就在该区间内。我们将中轴元素 `nums[pivot]` 与右边界元素 `nums[high]` 进行比较，可能会有以下的三种情况：

第一种情况是 `nums[pivot] < nums[high]`。如下图所示，这说明 `nums[pivot]` 是最小值右侧的元素，因此我们可以忽略二分查找区间的右半部分。

![](assets/0154_find-minimum-in-rotated-sorted-array-ii2.png)

第二种情况是 `nums[pivot] > nums[high]`。如下图所示，这说明 `nums[pivot]` 是最小值左侧的元素，因此我们可以忽略二分查找区间的左半部分。

![](assets/0154_find-minimum-in-rotated-sorted-array-ii3.png)

第三种情况是 `nums[pivot] == nums[high]`。如下图所示，由于重复元素的存在，我们并不能确定 `nums[pivot]` 究竟在最小值的左侧还是右侧，因此我们不能莽撞地忽略某一部分的元素。我们唯一可以知道的是，由于它们的值相同，所以无论 `nums[high]` 是不是最小值，都有一个它的「替代品」`nums[pivot]`，因此我们可以忽略二分查找区间的右端点。

![](assets/0154_find-minimum-in-rotated-sorted-array-ii4.png)

> 这个图没有想到。

当二分查找结束时，我们就得到了最小值所在的位置。

```go
func findMin(nums []int) int {
    low, high := 0, len(nums) - 1
    for low < high {
        pivot := low + (high - low) / 2
        if nums[pivot] < nums[high] {
            high = pivot
        } else if nums[pivot] > nums[high] {
            low = pivot + 1
        } else {
            high--
        }
    }
    return nums[low]
}
```

- 时间复杂度：平均时间复杂度为 $O(\log n)$，其中 n 是数组 nums 的长度。如果数组是随机生成的，那么数组中包含相同元素的概率很低，在二分查找的过程中，大部分情况都会忽略一半的区间。而在最坏情况下，如果数组中的元素完全相同，那么 while 循环就需要执行 n 次，每次忽略区间的右端点，时间复杂度为 $O(n)$。
- 空间复杂度：$O(1)$。

链接：https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array-ii/solution/xun-zhao-xuan-zhuan-pai-xu-shu-zu-zhong-de-zui--16/