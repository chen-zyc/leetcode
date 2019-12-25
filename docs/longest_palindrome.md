- [最长回文子串](#%e6%9c%80%e9%95%bf%e5%9b%9e%e6%96%87%e5%ad%90%e4%b8%b2)
  - [摘要](#%e6%91%98%e8%a6%81)
  - [解决方案](#%e8%a7%a3%e5%86%b3%e6%96%b9%e6%a1%88)
    - [方法一：最长公共子串](#%e6%96%b9%e6%b3%95%e4%b8%80%e6%9c%80%e9%95%bf%e5%85%ac%e5%85%b1%e5%ad%90%e4%b8%b2)
    - [方法二：暴力法](#%e6%96%b9%e6%b3%95%e4%ba%8c%e6%9a%b4%e5%8a%9b%e6%b3%95)
    - [方法三：动态规划](#%e6%96%b9%e6%b3%95%e4%b8%89%e5%8a%a8%e6%80%81%e8%a7%84%e5%88%92)
    - [方法四：中心扩展算法](#%e6%96%b9%e6%b3%95%e5%9b%9b%e4%b8%ad%e5%bf%83%e6%89%a9%e5%b1%95%e7%ae%97%e6%b3%95)
    - [方法五：Manacher 算法](#%e6%96%b9%e6%b3%95%e4%ba%94manacher-%e7%ae%97%e6%b3%95)



# 最长回文子串

[链接](https://leetcode-cn.com/problems/longest-palindromic-substring)

给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。

示例 1：

```
输入: "babad"
输出: "bab"
注意: "aba" 也是一个有效答案。
```

示例 2：

```
输入: "cbbd"
输出: "bb"
```

链接：https://leetcode-cn.com/problems/longest-palindromic-substring/solution/zui-chang-hui-wen-zi-chuan-by-leetcode/

## 摘要

这篇文章是为中级读者而写的。它介绍了回文，动态规划以及字符串处理。请确保你理解什么是回文。回文是一个正读和反读都相同的字符串，例如，“aba” 是回文，而 “abc” 不是。

## 解决方案

### 方法一：最长公共子串

**常见错误**

有些人会忍不住提出一个快速的解决方案，不幸的是，这个解决方案有缺陷(但是可以很容易地纠正)：

> 反转 S，使之变成 S'。找到 S 和 S' 之间最长的公共子串，这也必然是最长的回文子串。

这似乎是可行的，让我们看看下面的一些例子。

例如，S = "caba", S' = "abac"： 

S 以及 S' 之间的最长公共子串为 "aba"，恰恰是答案。

让我们尝试一下这个例子：S = "abacdfgdcaba", S' = "abacdgfdcaba"： 

S 以及 S' 之间的最长公共子串为 "abacd"。显然，这不是回文。

**算法**

我们可以看到，**当 S 的其他部分中存在非回文子串的反向副本时，最长公共子串法就会失败**。为了纠正这一点，每当我们找到最长的公共子串的候选项时，都**需要检查子串的索引是否与反向子串的原始索引相同**。如果相同，那么我们尝试更新目前为止找到的最长回文子串；如果不是，我们就跳过这个候选项并继续寻找下一个候选。

这给我们提供了一个复杂度为 $O(n^2)$ 动态规划解法，它将占用 $O(n^2)$ 的空间（可以改进为使用 $O(n)$ 的空间）。请在 [这里](https://baike.baidu.com/item/%E6%9C%80%E9%95%BF%E5%85%AC%E5%85%B1%E5%AD%90%E4%B8%B2/22799982?fr=aladdin) 阅读更多关于最长公共子串的内容。

### 方法二：暴力法

很明显，暴力法将选出所有子字符串可能的开始和结束位置，并检验它是不是回文。

复杂度分析

时间复杂度：$O(n^3)$，假设 n 是输入字符串的长度，则 $\binom{n}{2} = \frac{n(n-1)}{2}$ 为此类子字符串(不包括字符本身是回文的一般解法)的总数。因为验证每个子字符串需要 $O(n)$ 的时间，所以运行时间复杂度是 $O(n^3)$。

空间复杂度：$O(1)$。

### 方法三：动态规划

为了改进暴力法，我们首先观察如何避免在验证回文时进行不必要的重复计算。考虑 "ababa" 这个示例。如果我们已经知道 "bab" 是回文，那么很明显，"ababa" 一定是回文，因为它的左首字母和右尾字母是相同的。

我们给出 $P(i,j)$ 的定义如下：

$$
P(i,j) = \begin{cases} \text{true,} &\quad\text{如果子串} S_i \dots S_j \text{是回文子串}\\ \text{false,} &\quad\text{其它情况} \end{cases}
$$

因此，

$$
P(i, j) = ( P(i+1, j-1) \text{ and } S_i == S_j )
$$


基本示例如下：

$$
P(i, i) = true
$$
$$
P(i, i+1) = ( S_i == S_{i+1} )
$$

这产生了一个直观的动态规划解法，我们首先初始化一字母和二字母的回文，然后找到所有三字母回文，并依此类推…

**复杂度分析**

时间复杂度：$O(n^2)$，这里给出我们的运行时间复杂度为 $O(n^2)$ 。

空间复杂度：$O(n^2)$，该方法使用 $O(n^2)$ 的空间来存储表。

补充练习: 你能进一步优化上述解法的空间复杂度吗？

### 方法四：中心扩展算法

事实上，只需使用恒定的空间，我们就可以在 $O(n^2)$ 的时间内解决这个问题。

我们观察到回文中心的两侧互为镜像。因此，回文可以从它的中心展开，并且只有 $2n - 1$ 个这样的中心。

你可能会问，为什么会是 $2n - 1$ 个，而不是 n 个中心？原因在于所含字母数为偶数的回文的中心可以处于两字母之间（例如 "abba" 的中心在两个 'b' 之间）。

$$
\begin{array}{ccccc}
  & \Downarrow &  & \Downarrow & \\
1 &   & 2 & & 3
\end{array}
$$

```Java
public String longestPalindrome(String s) {
    if (s == null || s.length() < 1) return "";
    // 截取的范围应该是 [start, end]
    int start = 0, end = 0;
    for (int i = 0; i < s.length(); i++) {
        int len1 = expandAroundCenter(s, i, i);
        int len2 = expandAroundCenter(s, i, i + 1);
        int len = Math.max(len1, len2);
        // end - start + 1 才是长度
        // len >= end - start + 1 ==> len > end - start
        if (len > end - start) {
            start = i - (len - 1) / 2;
            end = i + len / 2;
        }
    }
    return s.substring(start, end + 1);
}

// 返回的是相等的字符的个数
private int expandAroundCenter(String s, int left, int right) {
    int L = left, R = right;
    while (L >= 0 && R < s.length() && s.charAt(L) == s.charAt(R)) {
        L--;
        R++;
    }
    return R - L - 1;
}
```

$$
\begin{array}{cccc}
& i & j & \\
\Leftarrow & | & | & \Rightarrow \\
0 & 1 & 2 & 3 \\
\end{array}
$$

偶数的情况下，len 也是偶数。$i = i - \frac{len-2}{2}$, `-2` 是因为要去掉 i 和 j。因为 len 是偶数，所以 $\frac{len-2}{2}$ 和 $\frac{len-1}{2}$ 相等，最终 $i = i - \frac{len-1}{2}$. 而 $j = i + 1 + \frac{len-2}{2} = i + \frac{len}{2}$.


$$
\begin{array}{cccc}
& i && \\
\Leftarrow & | & \Rightarrow &\\
0 & 1 & 2 & 3
\end{array}
$$

奇数的情况下，len 也是奇数。$i = i - \frac{len-1}{2}$, `-1` 是因为要去掉 i (j 和 i 是一样的位置）。最终 $i = i - \frac{len-1}{2}$. 而 $j = i  + \frac{len-1}{2}$, 因为 len 是奇数，所以 $\frac{len-1}{2} = \frac{len}{2}$, 所以 $j = i + \frac{len}{2}$.

最终上面两种情况可以统一成：

$$
\begin{array}{rcl}
i & = & i - \frac{len-1}{2} \\
j & = & i + \frac{len}{2}
\end{array}
$$

复杂度分析

时间复杂度：$O(n^2)$，由于围绕中心来扩展回文会耗去 $O(n)$ 的时间，所以总的复杂度为 $O(n^2)$。

空间复杂度：$O(1)$。

### 方法五：Manacher 算法

还有一个复杂度为 $O(n)$ 的 Manacher 算法。然而，这是一个非同寻常的算法，在 45 分钟的编码时间内提出这个算法将会是一个不折不扣的挑战。理解它，我保证这将是非常有趣的。
