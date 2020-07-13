- [350. 两个数组的交集 II](#350-两个数组的交集-ii)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：哈希表](#方法一哈希表)
    - [方法二：排序](#方法二排序)

------------------------------

# 350. 两个数组的交集 II

## 题目

给定两个数组，编写一个函数来计算它们的交集。

示例 1:

```
输入: nums1 = [1,2,2,1], nums2 = [2,2]
输出: [2,2]
```

示例 2:

```
输入: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出: [4,9]
```

说明：

- 输出结果中每个元素出现的次数，应与元素在两个数组中出现的次数一致。
- 我们可以不考虑输出结果的顺序。

进阶:

- 如果给定的数组已经排好序呢？你将如何优化你的算法？
- 如果 nums1 的大小比 nums2 小很多，哪种方法更优？
- 如果 nums2 的元素存储在磁盘上，磁盘内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/intersection-of-two-arrays-ii
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

这个题似曾相识，和 [最长重复子数组](docs/no_0718_maximum_length_of_repeated_subarray.md) 是不是一样的？

不一样的点：
- `[1, 2]` 和 `[2, 1]` 的交集是 `[1, 2]`。也就是说，数组的顺序是可以打乱的，只要求出现即可。
- 另外这道题也不要求是连续的。


> 链接：https://leetcode-cn.com/problems/intersection-of-two-arrays-ii/solution/liang-ge-shu-zu-de-jiao-ji-ii-by-leetcode-solution/

### 方法一：哈希表

由于同一个数字在两个数组中都可能出现多次，因此需要用哈希表存储每个数字出现的次数。对于一个数字，其在交集中出现的次数等于该数字在两个数组中出现次数的最小值。

首先遍历第一个数组，并在哈希表中记录第一个数组中的每个数字以及对应出现的次数，然后遍历第二个数组，对于第二个数组中的每个数字，如果在哈希表中存在这个数字，则将该数字添加到答案，并减少哈希表中该数字出现的次数。

为了降低空间复杂度，首先遍历较短的数组并在哈希表中记录每个数字以及对应出现的次数，然后遍历较长的数组得到交集。

![](assets/no_0350_intersection_of_two_arrays_ii1.gif)

```go
func intersect(nums1 []int, nums2 []int) []int {
    if len(nums1) > len(nums2) {
        return intersect(nums2, nums1)
    }
    m := map[int]int{}
    for _, num := range nums1 {
        m[num]++
    }

    intersection := []int{}
    for _, num := range nums2 {
        if m[num] > 0 {
            intersection = append(intersection, num)
            m[num]--
        }
    }
    return intersection
}
```

**复杂度分析**

- 时间复杂度：$O(m+n)$，其中 m 和 n 分别是两个数组的长度。需要遍历两个数组并对哈希表进行操作，哈希表操作的时间复杂度是 $O(1)$，因此总时间复杂度与两个数组的长度和呈线性关系。
- 空间复杂度：$O(\min(m,n))$，其中 m 和 n 分别是两个数组的长度。对较短的数组进行哈希表的操作，哈希表的大小不会超过较短的数组的长度。为返回值创建一个数组 intersection，其长度为较短的数组的长度。


### 方法二：排序

如果两个数组是有序的，则可以便捷地计算两个数组的交集。

首先对两个数组进行排序，然后使用两个指针遍历两个数组。

初始时，两个指针分别指向两个数组的头部。每次比较两个指针指向的两个数组中的数字，如果两个数字不相等，则**将指向较小数字的指针右移一位**，如果两个数字相等，将该数字添加到答案，并将两个指针都右移一位。当至少有一个指针超出数组范围时，遍历结束。

```go
func intersect(nums1 []int, nums2 []int) []int {
    sort.Ints(nums1)
    sort.Ints(nums2)
    length1, length2 := len(nums1), len(nums2)
    index1, index2 := 0, 0

    intersection := []int{}
    for index1 < length1 && index2 < length2 {
        if nums1[index1] < nums2[index2] {
            index1++
        } else if nums1[index1] > nums2[index2] {
            index2++
        } else {
            intersection = append(intersection, nums1[index1])
            index1++
            index2++
        }
    }
    return intersection
}
```

**复杂度分析**

- 时间复杂度：$O(m \log m+n \log n)$，其中 m 和 n 分别是两个数组的长度。对两个数组进行排序的时间复杂度是 $O(m \log m+n \log n)$，遍历两个数组的时间复杂度是 $O(m+n)$，因此总时间复杂度是 $O(m \log m+n \log n)$。
- 空间复杂度：$O(\min(m,n))$，其中 m 和 n 分别是两个数组的长度。为返回值创建一个数组 intersection，其长度为较短的数组的长度。不过在 C++ 中，我们可以直接创建一个 vector，不需要把答案临时存放在一个额外的数组中，所以这种实现的空间复杂度为 $O(1)$。

**结语**

如果 nums2 的元素存储在磁盘上，磁盘内存是有限的，并且你不能一次加载所有的元素到内存中。那么就无法高效地对 nums2 进行排序，因此推荐使用方法一而不是方法二。在方法一中，nums2 只关系到查询操作，因此每次读取 nums2 中的一部分数据，并进行处理即可。

