- [面试题 16.11. 跳水板](#面试题-1611-跳水板)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：数学](#方法一数学)

------------------------------

# 面试题 16.11. 跳水板

## 题目

你正在使用一堆木板建造跳水板。有两种类型的木板，其中长度较短的木板长度为shorter，长度较长的木板长度为longer。你必须正好使用k块木板。编写一个方法，生成跳水板所有可能的长度。

返回的长度需要从小到大排列。

示例：

```
输入：
shorter = 1
longer = 2
k = 3
输出： {3,4,5,6}
```

提示：

- 0 < shorter <= longer
- 0 <= k <= 100000

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/diving-board-lcci
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

- 长度可能会重复，所以还得去重。所以怎么去重呢？
- 用递归的算法会超时，而下面的方法有点做数学题的意思了。



### 方法一：数学

> 链接：https://leetcode-cn.com/problems/diving-board-lcci/solution/tiao-shui-ban-by-leetcode-solution/

首先考虑两种边界情况。

- 如果 k=0，则不能建造任何跳水板，因此返回空数组。
- 如果 shorter 和 longer 相等，则建造的跳水板的长度是唯一的，都等于 `shorter*k`，因此返回长度为 1 的数组，数组中的元素为 `shorter*k`。

然后考虑一般情况，即 `shorter<longer` 且 `k>0`。由于短木板和长木板一共使用 k 块，因此**一共有 `k+1` 种组合**(这个是怎么算出来的？)，每种组合下建造的跳水板长度都是不一样的，一共有 `k+1` 种不同的长度。

> 为什么有 k+1 种组合？设短木板有 i 块，长木板就有 k-i 块，而 i 取值为 `[0, k]`，所以共有 k+1 种组合。

为什么每种组合下建造的跳水板长度都是不一样的？考虑以下两种不同的组合：第一种组合，有 `i` 块长木板，则跳水板的长度是 `shorter*(k-i)+longer*i`；第二种组合，有 j 块长木板，则跳水板的长度是 `shorter*(k-j)+longer*j`。其中 $0 \leq i<j \leq k$。则两种不同的组合下的跳水板长度之差为：

$$
(shorter*(k-i)+longer*i)-(shorter*(k-j)+longer*j)=(longer-shorter)*(i-j)
$$

由于 `longer>shorter` 且 `i<j`，因此上式的值小于 0。由此可见，**任意两种不同的组合下的跳水板长度都是不一样的**，而且使用的长木板越多，跳水板的长度越大。

因此创建长度为 `k+1` 的数组 `lengths`，对于 $0 \leq i \leq k$，令 `lengths[i]=shorter∗(k−i)+longer∗i`，则 `lengths` 包含跳水板所有可能的长度，且长度为升序排序。


![](assets/interview_16_11_diving_board_lcci.gif)

> 最小的时候是 k 块短木板，然后每次都是去掉一块短木板然后加上一块长木板，所以每次的长度总比之前的要大。

```go
func divingBoard(shorter int, longer int, k int) []int {
    if k == 0 {
        return []int{}
    }
    if shorter == longer {
        return []int{shorter * k}
    }
    lengths := make([]int, k + 1)
    for i := 0; i <= k; i++ {
        lengths[i] = shorter * (k - i) + longer * i
    }
    return lengths
}
```

**复杂度分析**

- 时间复杂度：`O(k)`，其中 k 是木板数量。短木板和长木板一共使用 k 块，一共有 `k+1` 种组合，对于每种组合都要计算跳水板的长度。
- 空间复杂度：`O(1)`。除了返回值以外，额外使用的空间复杂度为常数。(返回值就不算在空间复杂度里吗？)
