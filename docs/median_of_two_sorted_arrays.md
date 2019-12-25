
## Approach 1: Recursive Approach

To solve this problem, we need to understand "What is the use of median". In statistics, the median is used for:

> Dividing a set into two equal length subsets, that one subset is always greater than the other.

中间值就是把集合分成长度相同的两部分，并且其中一部分的元素都比另一部分的大。

If we understand the use of median for dividing, we are very close to the answer.

First let's cut `A` into two parts at a random position `i`: (使用 i 把数组 A 切分成两部分)

```
          left_A             |        right_A
    A[0], A[1], ..., A[i-1]  |  A[i], A[i+1], ..., A[m-1]
```

Since `A` has `m` elements, so there are `m+1` kinds of cutting (`i=0∼m`).
`A` 有 m 个元素，所以一共有 m+1 种切法。

And we know:

```
len(left_A) = i, len(right_A) = m - i
Note: when i = 0, left_A is empty, and when i = m, right_A is empty.
```

`left_A` 的长度是 i， `right_A` 的长度是 m-1，当 i=0 时 `left_A` 时空的，当 i=m 时 `right_A` 是空的。

With the same way, cut `B` into two parts at a random position `j`:

```
          left_B             |        right_B
    B[0], B[1], ..., B[j-1]  |  B[j], B[j+1], ..., B[n-1]
```

同样地，使用 j 将数组 B 也划分成两个部分。

Put `left_A` and `left_B` into one set, and put `right_A` and `right_B` into another set. Let's name them `left_part` and `right_part`:

```
          left_part          |        right_part
    A[0], A[1], ..., A[i-1]  |  A[i], A[i+1], ..., A[m-1]
    B[0], B[1], ..., B[j-1]  |  B[j], B[j+1], ..., B[n-1]
```

把 A 的前部分和 B 的前部分放到一起，当做 `left_part`，把后一部分当做 `right_part`。

If we can ensure:

```
len(left_part) = len(right_part)
max(left_part) <= min(right_part)
```

then we divide all elements in {A,B} into two parts with equal length, and one part is always greater than the other. Then

median= $\frac{max(left\_part)+min(right\_part)}{2}$

如果经过划分后，`left_part` 和 `right_part` 的长度相等，并且 `left_part` 中的最大值 <= `right_part` 的最小值，那么中间值就是 `(max(left_part) + min(right_part))/2`。（假设我们把 {A, B} 重新排序，那么 `left_part` 就是排序后的数组的前半部分，`right_part` 就是后半部分）

To ensure these two conditions, we just need to ensure:

```
1. i + j = m − i + n − j (or: m - i + n - j + 1) // len(left_A) + len(left_B) = len(right_A) + len(right_B)
   if n >= m, we just need to set: i = 0 ~ m, j = (m + n + 1)/2 - i
2. B[j-1] <= A[i] and A[i-1] <= B[j]
```

ps.1 For simplicity, I presume `A[i−1],B[j−1],A[i],B[j]` are always valid even if i=0, i=m, j=0, or j=n. I will talk about how to deal with these edge values at last.

ps.2 Why n >= m? Because I have to make sure j is non-negative since `0 <= i <= m` and `j = ​(m+n+1)/2-i`. If n < m, then j may be negative, that will lead to wrong result.

公式一是保证划分后的两部分的长度相等，为什么可以是 `m-i+n-j+1` 呢？这里的 1 是为了当 m+n 是奇数时，j 会向后取整，比如 n=3,m=2,i=1 时 j=2，这样 `left_part` 有 3 个元素，而 `right_part` 有两个元素。

为什么要保证 n >= m 呢？如果 n < m, 比如 n=2, m=3，当 i = 0 时 j = 3，但是 j 最大只能取 2.

So, all we need to do is:

```
Searching i in [0, m], to find an object i such that:
    B[j−1] <= A[i] and A[i−1] <= B[j], where j = (m + n + 1)/2 - i
```

所以我们只要在 0-m 中找到一个 i，让它满足 `B[j-1] <= A[i] && A[i-1] <= B[j]` 就可以了。

And we can do a binary search following steps described below:

1. Set `imin=0, imax=m`, then start searching in `[imin,imax]`
2. Set `i = (imin + imax) / 2, j = (m + n + 1) / 2 - i`
3. Now we have `len(left_part)=len(right_part)`. And there are only 3 situations that we may encounter:
    1. `B[j − 1] <= A[i]` and `A[i − 1] <= B[j]`
        Means we have found the object i, so stop searching.
    2. `B[j − 1] > A[i]`
        Means `A[i]` is too small. We must adjust i to get `B[j − 1] <= A[i]`.
        Can we increase i?
            Yes. Because when i is increased, j will be decreased.
            So `B[j−1]` is decreased and `A[i]` is increased, and `B[j−1] <= A[i]` may be satisfied.
        Can we decrease i?
            No! Because when i is decreased, j will be increased.
            So `B[j−1]` is increased and `A[i]` is decreased, and `B[j−1] <= A[i]` will be never satisfied.
        So we must increase i. That is, we must adjust the searching range to `[i+1, imax]`.
        So, set `imin = i+1`, and goto 2.
    3. `A[i − 1] > B[j]`:
        Means `A[i−1]` is too big. And we must decrease i to get `A[i−1] <= B[j]`.
        That is, we must adjust the searching range to `[imin, i−1]`.
        So, set `imax = i−1`, and goto 2.

初始时设置 i 的范围在 `[0, m]`，然后计算 j，这样划分的两部分就满足 `len(left_part) = len(right_part)`，接下来就只需要判断是不是满足 `B[j-1] <= A[i] and A[i-1] <= B[j]` 就可以了。分三种情况：
1. 满足，那么这个 i 就是我们的解。
2. `B[j-1] > A[i]`：我们期望的是小于，这说明 `A[i]` 有点小，我们需要调整 i 的大小。如果把 i 调小点，那么 j 就会增大，`B[j-1]` 就更比 `A[i]` 大了，所以只能把 i 调大点。所以下个循环就把 i 的范围设置在 `[i+1, imax]`。
3. `A[i − 1] > B[j]`：同理，这说明 `A[i-1]` 大了，把 i 调小点，范围设置在 `[imin, i-1]`。

When the object i is found, the median is:

1. `max(A[i−1],B[j−1])`, when m + n is odd
2. `( max(A[i−1],B[j−1]) + min(A[i],B[j]) ) / 2`, when m + n is even

如果 i 已经找到了，那么中间值就是：
1. 如果 m+n 是奇数，`median = max(A[i-1], B[j-1])`
2. 如果 m+n 是偶数，`median =( max(A[i−1],B[j−1]) + min(A[i],B[j]) ) / 2` 

Now let's consider the edges values `i=0,i=m,j=0,j=n` where `A[i−1],B[j−1],A[i],B[j]` may not exist. Actually this situation is easier than you think.
当 `i=0,i=m,j=0,j=n` 时, `A[i−1],B[j−1],A[i],B[j]` 可能不存在，所以需要处理下边界值。

What we need to do is ensuring that `max(left_part) <= min(right_part)`. So, if i and j are not edges values (means `A[i−1],B[j−1],A[i],B[j]` all exist), then we must check both `B[j−1] <= A[i] and A[i−1] <= B[j]`. But if some of `A[i−1],B[j−1],A[i],B[j]` don't exist, then we don't need to check one (or both) of these two conditions. For example, if `i=0`, then `A[i−1]` doesn't exist, then we don't need to check `A[i−1] <= B[j]`. So, what we need to do is:

我们需要确定的是 `max(left_part) <= min(right_part)`。如果 `i=0`，那么 `A[i-1]` 就不存在，也就不需要判断 `A[i-1] <= B[j]` 的情况了。

```
Searching i in `[0, m]`, to find an object i such that:
(j=0 or i=m or B[j−1] <= A[i]) and (i=0 or j=n or A[i−1] <= B[j]), where j = (m+n+1)/2-i
```

And in a searching loop, we will encounter only three situations:

1. `j=0 or i=m or B[j−1] <= A[i])` and `i=0 or j=n or A[i−1] <= B[j])`: Means i is perfect, we can stop searching.
2. `j>0 and i<m and B[j−1]>A[i]`: Means i is too small, we must increase it.
3. `i>0 and j<n and A[i−1]>B[j]`: Means i is too big, we must decrease it.

Thanks to @Quentin.chen for pointing out that: `i < m` 可以推导出 `j > 0`, `i > 0` 可以推导出 `j < n`，因为：

```
m <= n, i < m ===> j = (m+n+1)/2-i > (m+n+1)/2-m >= (2m+1)/2-m >= 0
m <= n, i > 0 ===> j = (m+n+1)/2-i < (m+n+1)/2 <= (2n+1)/2 <= n
```

So in situation 2. and 3. , we don't need to check whether `j > 0` and whether `j < n`.

java 代码实现：

```java
class Solution {
    public double findMedianSortedArrays(int[] A, int[] B) {
        int m = A.length;
        int n = B.length;
        if (m > n) { // to ensure m<=n
            int[] temp = A; A = B; B = temp;
            int tmp = m; m = n; n = tmp;
        }
        int iMin = 0, iMax = m, halfLen = (m + n + 1) / 2;
        while (iMin <= iMax) {
            int i = (iMin + iMax) / 2;
            int j = halfLen - i;
            if (i < iMax && B[j-1] > A[i]){
                iMin = i + 1; // i is too small
            }
            else if (i > iMin && A[i-1] > B[j]) {
                iMax = i - 1; // i is too big
            }
            else { // i is perfect
                int maxLeft = 0;
                if (i == 0) { maxLeft = B[j-1]; }
                else if (j == 0) { maxLeft = A[i-1]; }
                else { maxLeft = Math.max(A[i-1], B[j-1]); }
                if ( (m + n) % 2 == 1 ) { return maxLeft; }

                int minRight = 0;
                if (i == m) { minRight = B[j]; }
                else if (j == n) { minRight = A[i]; }
                else { minRight = Math.min(B[j], A[i]); }

                return (maxLeft + minRight) / 2.0;
            }
        }
        return 0.0;
    }
}
```

