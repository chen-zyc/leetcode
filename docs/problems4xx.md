- [](#)
- [476. 数字的补数](#476-数字的补数)


------------------------------

# 453. 最小操作次数使数组元素相等

给你一个长度为 n 的整数数组，每次操作将会使 n - 1 个元素增加 1 。返回让数组所有元素相等的最小操作次数。

示例 1：

```
输入：nums = [1,2,3]
输出：3
解释：
只需要3次操作（注意每次操作会增加两个元素的值）：
[1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
```

示例 2：

```
输入：nums = [1,1,1]
输出：0
```

提示：

- n == nums.length
- $1 <= nums.length <= 10^5$
- $-10^9 <= nums[i] <= 10^9$
- 答案保证符合 32-bit 整数

[链接](https://leetcode-cn.com/problems/minimum-moves-to-equal-array-elements)

我用最笨的方法做的，但是**超时**了。下面是提交的代码：

```rs
pub fn min_moves(mut nums: Vec<i32>) -> i32 {
    let mut count = 0;
    loop {
        if move_once(&mut nums) {
            count += 1;
        } else {
            break;
        }
    }
    count
}

fn move_once(nums: &mut Vec<i32>) -> bool {
    // 找出最大值，以及最大值出现的次数。
    // 如果所有的值都相等，那么就不需要改变了。
    // 否则，将除最大值(多个最大值只取其中一个)之外的数都加一。

    // 这个的一个优化是：可以只遍一次，把所有的元素都加一，同时记录下最大值，最后再把它减一。

    let (mut max_value, mut max_value_idx, mut max_value_count) = (i32::MIN, 0, 0);
    for (idx, num) in nums.iter().enumerate() {
        if *num > max_value {
            max_value = *num;
            max_value_idx = idx;
            max_value_count = 1;
        } else if *num == max_value {
            max_value_count += 1;
        }
    }
    if max_value_count == nums.len() {
        return false;
    }
    for (idx, num) in nums.iter_mut().enumerate() {
        if idx != max_value_idx {
            *num += 1;
        }
    }
    println!("{:?}, {}", nums, max_value_count);
    true
}
```

看题解有点像是脑筋急转弯。以下是**官方题解**：

因为只需要找出让数组所有元素相等的最小操作次数，所以我们不需要考虑数组中各个元素的绝对大小，即**不需要真正算出数组中所有元素相等时的元素值**(_我的方法就是属于算出了最后的那个值_)，只需要考虑数组中元素相对大小的变化即可。

因此，每次操作既可以理解为使 n-1 个元素增加 1，**也可以理解使 1 个元素减少 1**(_这个真是没想到呀_)。显然，后者更利于我们的计算。

于是，要计算让数组中所有元素相等的操作数，我们只需要计算将数组中所有元素都减少到数组中元素最小值所需的操作数，即计算 $\left( \sum_{i=0}^{n-1} \textit{nums}[i] \right) - min(\textit{nums}) * n$ (_我在那个求和的两边加了个扩号，原文是没有的，导致我理解成最后才求和了_)

其中 n 为数组 nums 的长度，min(nums)为数组 nums 中元素的最小值。

在实现中，为避免溢出，我们可以逐个累加每个元素与数组中元素最小值的差，即计算 $\sum_{i=0}^{n-1} (\textit{nums}[i] - \textit{min}(\textit{nums}))$

[链接](https://leetcode-cn.com/problems/minimum-moves-to-equal-array-elements/solution/zui-xiao-cao-zuo-ci-shu-shi-shu-zu-yuan-3meg3/)

根据上面的题解写的答案：

```rs
pub fn min_moves(nums: Vec<i32>) -> i32 {
    let min = nums.iter().min().unwrap(); // nums 至少有一个，所以可以 unwrap()。
    nums.iter().fold(0, |acc, n| n - min + acc)
}
```


# 476. 数字的补数

对整数的二进制表示取反（0 变 1 ，1 变 0）后，再转换为十进制表示，可以得到这个整数的补数。

例如，整数 5 的二进制表示是 "101" ，取反后得到 "010" ，再转回十进制表示得到补数 2 。

给你一个整数 num ，输出它的补数。
 
示例 1：

```
输入：num = 5
输出：2
解释：5 的二进制表示为 101（没有前导零位），其补数为 010。所以你需要输出 2 。
```

示例 2：

```
输入：num = 1
输出：0
解释：1 的二进制表示为 1（没有前导零位），其补数为 0。所以你需要输出 0 。
```

提示：

- $1 <= num < 2^{31}$

链接：https://leetcode-cn.com/problems/number-complement


**解答**

取反直接使用 `!num` 就可以了，但是这个题目有个隐藏的条件，比如 5 的二进制是 `0b_101`，最左边的 1 的前面还有很多 0 呢，在取反的时候是不对这些 0 进行取反的。所以直接用 `!num` 是不行的。

考虑用异或，比如 `0b_101 ^ 0b_111`，0 和 0 异或的结果还是 0，1 和 1 异或的结果是 0，0 和 1 异或的结果是 1，这样就达到了把 1 变成 0，把 0 变成 1 的效果了，所以现在的问题是怎么求出 `0b_111` 这个数，也就是怎么求出对右边几位比特进行取反操作。

可以算出来 num 有几个前导零，然后剩下的数就是需要取反的位数了。

```rs
pub fn find_complement(num: i32) -> i32 {
    // 总的位数 - 前面 0 的位数 = 需要取反的位数
    let bits = i32::BITS - num.leading_zeros();
    // 只有后面 bits 位是 1。
    let mask = (1 << bits) - 1;
    println!(
        "num: {:b}, bits: {}, mask: {:b}, res: {:b}",
        num,
        bits,
        mask,
        num ^ mask
    );
    // num 和 mask 前面几位都是 0，所以异或的结果还是 0。后面几位 mask 是 1，异或的结果就是原来的 0 变成 1，1 变成 0。
    num ^ mask

    // 上面操作的简化版。
    // num ^ ((1 << (i32::BITS - num.leading_zeros())) - 1)
}
```