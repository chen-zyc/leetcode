- [349. 两个数组的交集](#349-两个数组的交集)
  - [官方题解](#官方题解)
    - [方法一：两个集合](#方法一两个集合)
    - [方法二：排序 + 双指针](#方法二排序--双指针)

------------------------------

# 349. 两个数组的交集

给定两个数组，编写一个函数来计算它们的交集。

示例 1：

```
输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]
```

示例 2：

```
输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
```
 

说明：

- 输出结果中的每个元素一定是唯一的。
- 我们可以不考虑输出结果的顺序。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/intersection-of-two-arrays
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/intersection-of-two-arrays/solution/liang-ge-shu-zu-de-jiao-ji-by-leetcode-solution/

### 方法一：两个集合

计算两个数组的交集，直观的方法是遍历数组 nums1，对于其中的每个元素，遍历数组 nums2 判断该元素是否在数组 nums2 中，如果存在，则将该元素添加到返回值。假设数组 nums1 和 nums2 的长度分别是 m 和 n，则遍历数组 nums1 需要 $O(m)$ 的时间，判断 nums1 中的每个元素是否在数组 nums2 中需要 $O(n)$ 的时间，因此总时间复杂度是 $O(mn)$。

如果使用哈希集合存储元素，则可以在 $O(1)$ 的时间内判断一个元素是否在集合中，从而降低时间复杂度。

首先使用两个集合分别存储两个数组中的元素，然后遍历较小的集合，判断其中的每个元素是否在另一个集合中，如果元素也在另一个集合中，则将该元素添加到返回值。该方法的时间复杂度可以降低到 $O(m+n)$。

```go
func intersection(nums1 []int, nums2 []int) (intersection []int) {
    set1 := map[int]struct{}{}
    for _, v := range nums1 {
        set1[v] = struct{}{}
    }
    set2 := map[int]struct{}{}
    for _, v := range nums2 {
        set2[v] = struct{}{}
    }
    if len(set1) > len(set2) {
        set1, set2 = set2, set1
    }
    for v := range set1 {
        if _, has := set2[v]; has {
            intersection = append(intersection, v)
        }
    }
    return
}
```

复杂度分析

- 时间复杂度：$O(m+n)$，其中 m 和 n 分别是两个数组的长度。使用两个集合分别存储两个数组中的元素需要 $O(m+n)$ 的时间，遍历较小的集合并判断元素是否在另一个集合中需要 $O(\min(m,n))$ 的时间，因此总时间复杂度是 $O(m+n)$。
- 空间复杂度：$O(m+n)$，其中 m 和 n 分别是两个数组的长度。空间复杂度主要取决于两个集合。

> 只把其中一个集合放进 set 中，然后遍历另一个数组，判断是不是在，在的话就从 set 中删除，这样下次再出现的话就不在 set 中了，这样不是更好？


### 方法二：排序 + 双指针

如果两个数组是有序的，则可以使用双指针的方法得到两个数组的交集。

首先对两个数组进行排序，然后使用两个指针遍历两个数组。可以预见的是加入答案的数组的元素一定是递增的，为了保证加入元素的唯一性，我们需要额外记录变量 `pre` 表示上一次加入答案数组的元素。

初始时，两个指针分别指向两个数组的头部。每次比较两个指针指向的两个数组中的数字，如果两个数字不相等，则将指向较小数字的指针右移一位，如果两个数字相等，且该数字不等于 `pre` ，将该数字添加到答案并更新 `pre` 变量，同时将两个指针都右移一位。当至少有一个指针超出数组范围时，遍历结束。

```go
func intersection(nums1 []int, nums2 []int) (res []int) {
    sort.Ints(nums1)
    sort.Ints(nums2)
    for i, j := 0, 0; i < len(nums1) && j < len(nums2); {
        x, y := nums1[i], nums2[j]
        if x == y {
            if res == nil || x > res[len(res)-1] {
                res = append(res, x)
            }
            i++
            j++
        } else if x < y {
            i++
        } else {
            j++
        }
    }
    return
}
```

复杂度分析

- 时间复杂度：$O(m \log m+n \log n)$，其中 m 和 n 分别是两个数组的长度。对两个数组排序的时间复杂度分别是 $O(m \log m)$ 和 $O(n \log n)$，双指针寻找交集元素的时间复杂度是 $O(m+n)$，因此总时间复杂度是 $O(m \log m+n \log n)$。
- 空间复杂度：$O(\log m+\log n)$，其中 m 和 n 分别是两个数组的长度。空间复杂度主要取决于排序使用的额外空间。
