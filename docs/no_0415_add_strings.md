- [415. 字符串相加](#415-字符串相加)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：模拟](#方法一模拟)

------------------------------

# 415. 字符串相加

## 题目

给定两个字符串形式的非负整数 num1 和num2 ，计算它们的和。

注意：

- num1 和num2 的长度都小于 5100.
- num1 和num2 都只包含数字 0-9.
- num1 和num2 都不包含任何前导零。
- 你不能使用任何內建 BigInteger 库， 也不能直接将输入的字符串转换为整数形式。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/add-strings
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/add-strings/solution/zi-fu-chuan-xiang-jia-by-leetcode-solution/

### 方法一：模拟

**思路与算法**

本题我们只需要对两个大整数模拟「竖式加法」的过程。竖式加法就是我们平常学习生活中常用的对两个整数相加的方法，回想一下我们在纸上对两个整数相加的操作，是不是如下图将相同数位对齐，从低到高逐位相加，如果当前位和超过 10，则向高位进一位？因此我们只要将这个过程用代码写出来即可。

具体实现也不复杂，我们定义两个指针 $i$ 和 $j$ 分别指向 $\textit{num1}$ 和 $\textit{num2}$ 的末尾，即最低位，同时定义一个变量 $\textit{add}$ 维护当前是否有进位，然后从末尾到开头逐位相加即可。你可能会想两个数字位数不同怎么处理，这里我们统一在指针当前下标处于负数的时候返回 0，等价于对位数较短的数字进行了补零操作，这样就可以除去两个数字位数不同情况的处理，具体可以看下面的代码。

```go
func addStrings(num1 string, num2 string) string {
    add := 0
    ans := ""
    for i, j := len(num1) - 1, len(num2) - 1; i >= 0 || j >= 0 || add != 0; i, j = i - 1, j - 1 {
        var x, y int
        if i >= 0 {
            x = int(num1[i] - '0')
        }
        if j >= 0 {
            y = int(num2[j] - '0')
        }
        result := x + y + add
        ans = strconv.Itoa(result%10) + ans
        add = result / 10
    }
    return ans
}
```

**复杂度分析**

- 时间复杂度：$O(\max(\textit{len}_1,\textit{len}_2))$，其中 $\textit{len}_1=\textit{num1}.\textit{length}$，$\textit{len}_2=\textit{num2}.\textit{length}$。竖式加法的次数取决于较大数的位数。
- 空间复杂度：$O(1)$。除答案外我们只需要常数空间存放若干变量。在 Java 解法中使用到了 StringBuffer，故 Java 解法的空间复杂度为 $O(n)$。
