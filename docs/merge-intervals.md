# 合并区间

[链接](https://leetcode-cn.com/problems/merge-intervals)


给出一个区间的集合，请合并所有重叠的区间。

示例 1:

```
输入: [[1,3],[2,6],[8,10],[15,18]]
输出: [[1,6],[8,10],[15,18]]
解释: 区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
```

示例 2:

```
输入: [[1,4],[4,5]]
输出: [[1,5]]
解释: 区间 [1,4] 和 [4,5] 可被视为重叠区间。
```

## 方法 2：排序

链接：https://leetcode-cn.com/problems/merge-intervals/solution/he-bing-qu-jian-by-leetcode/

**直觉**

如果我们按照区间的 start 大小排序，那么在这个排序的列表中可以合并的区间一定是连续的。

**算法**

首先，我们将列表按上述方式排序。然后，我们将第一个区间插入 merged 数组中，然后按顺序考虑之后的每个区间：如果当前区间的左端点在前一个区间的右端点之后，那么他们不会重合，我们可以直接将这个区间插入 merged 中；否则，他们重合，我们用当前区间的右端点更新前一个区间的右端点 end 如果前者数值比后者大的话。

一个简单的证明：假设算法在某些情况下没能合并两个本应合并的区间，那么说明存在这样的三元组 i，j 和 k 以及区间 ints 满足 $i < j < k$ 并且 $(ints[i], ints[k])$ 可以合并，而 $(ints[i], ints[j])$ 和 $(ints[j], ints[k])$ 不能合并。这说明满足下面的不等式：

$$
ints[i].end < ints[j].start \\ 
ints[j].end < ints[k].start \\ 
ints[i].end \geq ints[k].start \\
$$

我们联立这些不等式（注意还有一个显然的不等式 $ints[j].start \leq ints[j].end$），可以发现冲突：

```
ints[i].end < ints[j].start <= ints[j].end < ints[k].start 
                               ints[i].end >= ints[k].start
```	
 
 ![](assets/merge-intervals.png)

因此，**所有能够合并的区间必然是连续的**。

```java
class Solution {
    private class IntervalComparator implements Comparator<Interval> {
        @Override
        public int compare(Interval a, Interval b) {
            return a.start < b.start ? -1 : a.start == b.start ? 0 : 1;
        }
    }

    public List<Interval> merge(List<Interval> intervals) {
        Collections.sort(intervals, new IntervalComparator());

        LinkedList<Interval> merged = new LinkedList<Interval>();
        for (Interval interval : intervals) {
            // if the list of merged intervals is empty or if the current
            // interval does not overlap with the previous, simply append it.
            if (merged.isEmpty() || merged.getLast().end < interval.start) {
                merged.add(interval);
            }
            // otherwise, there is overlap, so we merge the current and previous
            // intervals.
            else {
                merged.getLast().end = Math.max(merged.getLast().end, interval.end);
            }
        }

        return merged;
    }
}
```

**复杂度分析**

时间复杂度：$O(n\log{}n)$

除去 sort 的开销，我们只需要一次线性扫描，所以主要的时间开销是排序的 $O(nlgn)$

空间复杂度：$O(1)$ (or $O(n)$)

如果我们可以原地排序 intervals ，就不需要额外的存储空间；否则，我们就需要一个线性大小的空间去存储 intervals 的备份，来完成排序过程。