- [167. 两数之和 II - 输入有序数组](#167-两数之和-ii---输入有序数组)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：二分查找](#方法一二分查找)
    - [方法二：双指针](#方法二双指针)

------------------------------

# 167. 两数之和 II - 输入有序数组

## 题目

给定一个已按照 **升序排列** 的有序数组，找到两个数使得它们相加之和等于目标数。

函数应该返回这两个下标值 index1 和 index2，其中 index1 必须小于 index2。

说明:

- 返回的下标值（index1 和 index2）不是从零开始的。(从 1 开始的？)
- 你可以假设每个输入只对应唯一的答案，而且你**不可以重复使用相同的元素**。

示例:

```
输入: numbers = [2, 7, 11, 15], target = 9
输出: [1,2]
解释: 2 与 7 之和等于目标数 9 。因此 index1 = 1, index2 = 2 。
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

我的思路：
> 先选第一个值，从最小的开始选，如果它大于 target 的一半就不用选第二个了，因为最小的都大于一半了，那另一个值肯定也大于一半，它们的和肯定已经超过 target 了。
> 选第二个值的话，可以使用二分查找，范围是 `[index+1 ..]`。
> 下面的双指针的解法没有想到。

--------------------

> 链接：https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/solution/liang-shu-zhi-he-ii-shu-ru-you-xu-shu-zu-by-leet-2/

**前言**

这道题可以使用 [两数之和](https://leetcode-cn.com/problems/two-sum/) 的解法，使用 $O(n^2)$ 的时间复杂度和 $O(1)$ 的空间复杂度暴力求解，或者借助哈希表使用 $O(n)$ 的时间复杂度和 $O(n)$ 的空间复杂度求解。但是这两种解法都是针对无序数组的，没有利用到输入数组有序的性质。利用输入数组有序的性质，可以得到时间复杂度和空间复杂度更优的解法。

### 方法一：二分查找

在数组中找到两个数，使得它们的和等于目标值，可以首先固定第一个数，然后寻找第二个数，第二个数等于目标值减去第一个数的差。利用数组的有序性质，可以通过二分查找的方法寻找第二个数。为了避免重复寻找，在寻找第二个数时，只在第一个数的右侧寻找。

```go
func twoSum(numbers []int, target int) []int {
    for i := 0; i < len(numbers); i++ {
        low, high := i + 1, len(numbers) - 1
        for low <= high {
            mid := (high - low) / 2 + low
            if numbers[mid] == target - numbers[i] {
                return []int{i + 1, mid + 1}
            } else if numbers[mid] > target - numbers[i] {
                high = mid - 1
            } else {
                low = mid + 1
            }
        }
    }
    return []int{-1, -1}
}
```

**复杂度分析**

- 时间复杂度：$O(n \log n)$，其中 n 是数组的长度。需要遍历数组一次确定第一个数，时间复杂度是 $O(n)$，寻找第二个数使用二分查找，时间复杂度是 $O(\log n)$，因此总时间复杂度是 $O(n \log n)$。
- 空间复杂度：$O(1)$。

### 方法二：双指针

初始时两个指针分别指向第一个元素位置和最后一个元素的位置。每次计算两个指针指向的两个元素之和，并和目标值比较。如果两个元素之和等于目标值，则发现了唯一解。如果两个元素之和小于目标值，则将左侧指针右移一位。如果两个元素之和大于目标值，则将右侧指针左移一位。移动指针之后，重复上述操作，直到找到答案。

使用双指针的实质是缩小查找范围。那么会不会把可能的解过滤掉？答案是不会。假设 $\text{numbers}[i]+\text{numbers}[j]=\text{target}$ 是唯一解，其中 $0 \leq i<j \leq \text{numbers.length}-1$。初始时两个指针分别指向下标 0 和下标 $\text{numbers.length}-1$，左指针指向的下标小于或等于 i，右指针指向的下标大于或等于 j。除非初始时左指针和右指针已经位于下标 i 和 j，否则一定是左指针先到达下标 i 的位置或者右指针先到达下标 j 的位置。

如果左指针先到达下标 i 的位置，此时右指针还在下标 j 的右侧，$\text{sum}>\text{target}$，因此一定是右指针左移，左指针不可能移到 i 的右侧。

如果右指针先到达下标 j 的位置，此时左指针还在下标 i 的左侧，$\text{sum}<\text{target}$，因此一定是左指针右移，右指针不可能移到 j 的左侧。

由此可见，在整个移动过程中，左指针不可能移到 i 的右侧，右指针不可能移到 j 的左侧，因此不会把可能的解过滤掉。由于题目确保有唯一的答案，因此使用双指针一定可以找到答案。

> 为什么这个解法是正确的可以参考[这个](https://leetcode-cn.com/problems/two-sum-ii-input-array-is-sorted/solution/yi-zhang-tu-gao-su-ni-on-de-shuang-zhi-zhen-jie-fa/)题解。

```go
func twoSum(numbers []int, target int) []int {
    low, high := 0, len(numbers) - 1
    for low < high {
        sum := numbers[low] + numbers[high]
        if sum == target {
            return []int{low + 1, high + 1}
        } else if sum < target {
            low++
        } else {
            high--
        }
    }
    return []int{-1, -1}
}
```

**复杂度分析**

- 时间复杂度：$O(n)$，其中 n 是数组的长度。两个指针移动的总次数最多为 n 次。
- 空间复杂度：$O(1)$。