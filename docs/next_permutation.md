# next_permutation

Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.

If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).

The replacement must be in-place, do not allocate extra memory.

Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
`1,2,3` → `1,3,2`
`3,2,1` → `1,2,3`
`1,1,5` → `1,5,1`



实现下一个排列，比原来的排列数值要大，如果不能更大的话就返回最小的顺序。

原址替换，不能分配额外内存。



来源：<https://discuss.leetcode.com/topic/2542/share-my-o-n-time-solution>

1. 从最后一个元素开始，向前遍历找到第一个索引`i`使得满足`num[i-1] < num[i]`,所以 `num[i]` 到 `num[n-1]`是逐渐减小的。
2. 为了找到下一个排列，我们需要在不同的位置交换一些数字，为了最小化增长的数量，我们需要尽可能的选择更高的更改的位置。注意到大于等于`i`的索引是不可能的，因为 `num[i, n-1]`是逐渐减小的。所以，我们想增加`i-1`位置的数值，把它和`num[i, n-1]`间的大于`num[i-1]`的最小的数值。例如，原数字`121543321`，我们想交换位置2的数字'1'和位置7的数字'2'。
3. 最后一步是使高位置的数值尽可能的小，我们只要把`num[i, n-1]`按从小到大排列就可以了。



