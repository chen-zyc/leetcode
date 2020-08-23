- [34. 在排序数组中查找元素的第一个和最后一个位置](#34-在排序数组中查找元素的第一个和最后一个位置)
  - [官方题解](#官方题解)
    - [方法 2：二分查找](#方法-2二分查找)
  - [其它解法](#其它解法)

------------------------------

# 34. 在排序数组中查找元素的第一个和最后一个位置

给定一个按照升序排列的整数数组 nums，和一个目标值 target。找出给定目标值在数组中的开始位置和结束位置。

你的算法时间复杂度必须是 $O(log n)$ 级别。

如果数组中不存在目标值，返回 `[-1, -1]`。

示例 1:

```
输入: nums = [5,7,7,8,8,10], target = 8
输出: [3,4]
```

示例 2:

```
输入: nums = [5,7,7,8,8,10], target = 6
输出: [-1,-1]
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/solution/zai-pai-xu-shu-zu-zhong-cha-zhao-yuan-su-de-di-yi-/

### 方法 2：二分查找

**想法**

因为数组已经排过序了，我们可以使用二分查找的方法去定位左右下标。

**算法**

总体算法工作过程与线性扫描方法类似，除了找最左和最右下标的方法。这里我们仅仅做几个微小的调整，用这种修改过的二分查找方法去搜索这个排过序的数组。首先，为了找到最左边（或者最右边）包含 target 的下标（而不是找到的话就返回 true ），所以算法在我们找到一个 target 后不能马上停止。我们需要继续搜索，直到 `lo == hi` 且它们在某个 target 值处下标相同。

另一个改变是 **left 参数的引入**，它是一个 boolean 类型的变量，指示我们在遇到 `target == nums[mid]` 时应该做什么。如果 left 为 true ，那么我们递归查询左区间，否则递归右区间。考虑如果我们在下标为 i 处遇到了 target ，最左边的 target 一定不会出现在下标大于 i 的位置，所以我们永远不需要考虑右子区间。当求最右下标时，道理同样适用。

```java
class Solution {
    // returns leftmost (or rightmost) index at which `target` should be
    // inserted in sorted array `nums` via binary search.
    private int extremeInsertionIndex(int[] nums, int target, boolean left) {
        int lo = 0;
        int hi = nums.length;

        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (nums[mid] > target || (left && target == nums[mid])) {
                hi = mid;
            }
            else {
                lo = mid+1;
            }
        }

        return lo;
    }

    public int[] searchRange(int[] nums, int target) {
        int[] targetRange = {-1, -1};

        int leftIdx = extremeInsertionIndex(nums, target, true);

        // assert that `leftIdx` is within the array bounds and that `target`
        // is actually in `nums`.
        if (leftIdx == nums.length || nums[leftIdx] != target) {
            return targetRange;
        }

        targetRange[0] = leftIdx;
        targetRange[1] = extremeInsertionIndex(nums, target, false)-1;

        return targetRange;
    }
}
```

**复杂度分析**

- 时间复杂度： $O(\log_{2}n)$ 。 
    由于二分查找每次将搜索区间大约划分为两等分，所以至多有 $\lceil \log_{2}n \rceil$ 次迭代。二分查找的过程被调用了两次，所以总的时间复杂度是对数级别的。 
- 空间复杂度：$O(1)$ 。

所有工作都是原地进行的，所以总的内存空间是常数级别的。


## 其它解法

我做的是用二分查找出任意一个 target，然后向左遍历找到最左，向右遍历找到最右。但这种有个问题是，如果所有元素都相同，那么复杂度就变成 $O(n)$ 了。

还有的是找 target-1 的最大索引，以及 target+1 的最小索引。这种其实和官方题解的思路差不多。