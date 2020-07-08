- [22. 括号生成](#22-括号生成)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：暴力法](#方法一暴力法)
    - [方法二：回溯法](#方法二回溯法)
    - [方法三：按括号序列的长度递归](#方法三按括号序列的长度递归)

------------------------------

# 22. 括号生成

## 题目

数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 **有效的** 括号组合。

示例：

```
输入：n = 3
输出：[
       "((()))",
       "(()())",
       "(())()",
       "()(())",
       "()()()"
     ]
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/generate-parentheses
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

递归形式的看代码。


非递归：<https://discuss.leetcode.com/topic/3474/an-iterative-method>

首先考虑怎样从之前的结果中(`f(0)...f(n)`)计算出`f(n)`。

事实上，`f(n)`的结果就是在`f(n-1)`的基础上添加额外的`()`。让`(`总是在首位，为了生成有效的结果，可以让`i`对括号在额外的括号之内，剩下的`n-1-i`对括号在额外的括号之外。

下面是一个例子：

```
f(0): ""

f(1): "("f(0)")"

f(2): "("f(0)")"f(1), "("f(1)")"

f(3): "("f(0)")"f(2), "("f(1)")"f(1), "("f(2)")"

So f(n) = "("f(0)")"f(n-1) , "("f(1)")"f(n-2) "("f(2)")"f(n-3) ... "("f(i)")"f(n-1-i) ... "(f(n-1)")"
```


> 链接：https://leetcode-cn.com/problems/generate-parentheses/solution/gua-hao-sheng-cheng-by-leetcode-solution/

### 方法一：暴力法

**思路**

我们可以生成所有 $2^{2n}$ 个 '(' 和 ')' 字符构成的序列，然后我们检查每一个是否有效即可。 

**算法**

为了生成所有序列，我们可以使用递归。长度为 n 的序列就是在长度为 n-1 的序列前加一个 '(' 或 ')'。

为了检查序列是否有效，我们遍历这个序列，并使用一个变量 balance 表示左括号的数量减去右括号的数量。如果在遍历过程中 balance 的值小于零，或者结束时 balance 的值不为零，那么该序列就是无效的，否则它是有效的。

```java
class Solution {
    public List<String> generateParenthesis(int n) {
        List<String> combinations = new ArrayList();
        generateAll(new char[2 * n], 0, combinations);
        return combinations;
    }

    public void generateAll(char[] current, int pos, List<String> result) {
        if (pos == current.length) {
            if (valid(current))
                result.add(new String(current));
        } else {
            // 在当前位置上生成 '(' 或 ')', 然后在剩下的位置上生成其它的。
            current[pos] = '(';
            generateAll(current, pos+1, result);
            current[pos] = ')';
            generateAll(current, pos+1, result);
        }
    }

    public boolean valid(char[] current) {
        // 左右括号的数量要相等。
        int balance = 0;
        for (char c: current) {
            if (c == '(') balance++;
            else balance--;
            if (balance < 0) return false;
        }
        return (balance == 0);
    }
}
```

**复杂度分析**

- 时间复杂度：$O(2^{2n}n)$，对于 $2^{2n}$ 个序列中的每一个，我们用于建立和验证该序列的复杂度为 $O(n)$。
- 空间复杂度：$O(n)$，除了答案数组之外，我们所需要的空间取决于递归栈的深度，每一层递归函数需要 $O(1)$ 的空间，最多递归 2n 层，因此空间复杂度为 $O(n)$。


### 方法二：回溯法

**思路和算法**

方法一还有改进的余地：我们可以**只在序列仍然保持有效时才添加 '(' or ')'**，而不是像 方法一 那样每次添加。我们可以通过跟踪到目前为止放置的左括号和右括号的数目来做到这一点，

**如果左括号数量不大于 n，我们可以放一个左括号。如果右括号数量小于左括号的数量，我们可以放一个右括号**。

```py
class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        ans = []
        def backtrack(S, left, right):
            # 如果括号的数量已经够了，那么就添加到结果中，因为 S 中总是保持有效的括号，所以可以直接添加。
            if len(S) == 2 * n:
                ans.append(''.join(S))
                return
            if left < n:
                # 如果左括号不够，就一直添加。这个有点想不到呀。
                S.append('(')
                backtrack(S, left+1, right)
                S.pop()  # 把左括号删除
            if right < left:
                # 左括号尝试完了，现在尝试右括号。
                # 右括号不能比左括号多
                S.append(')')
                backtrack(S, left, right+1)
                S.pop()

        backtrack([], 0, 0)
        return ans
```

**复杂度分析**

我们的复杂度分析依赖于理解 generateParenthesis(n) 中有多少个元素。这个分析超出了本文的范畴，但事实证明这是第 n 个卡特兰数 $\dfrac{1}{n+1}\dbinom{2n}{n}$，这是由 $\dfrac{4^n}{n\sqrt{n}}$ 渐近界定的。

- 时间复杂度：$O(\dfrac{4^n}{\sqrt{n}})$，在回溯过程中，每个答案需要 $O(n)$ 的时间复制到答案数组中。
- 空间复杂度：$O(n)$，除了答案数组之外，我们所需要的空间取决于递归栈的深度，每一层递归函数需要 $O(1)$ 的空间，最多递归 2n 层，因此空间复杂度为 $O(n)$。


### 方法三：按括号序列的长度递归

**思路与算法**

任何一个括号序列都一定是由 `(` 开头，并且第一个 `(` 一定有一个唯一与之对应的 `)`。这样一来，每一个括号序列可以用 `(a)b` 来表示，其中 a 与 b 分别是一个合法的括号序列（可以为空）。

那么，要生成所有长度为 `2 * n` 的括号序列，我们定义一个函数 `generate(n)` 来返回所有可能的括号序列。那么在函数 `generate(n)` 的过程中：

- 我们需要枚举与第一个 `(` 对应的 `)` 的位置 `2 * i + 1`；
- 递归调用 `generate(i)` 即可计算 a 的所有可能性；
- 递归调用 `generate(n - i - 1)` 即可计算 b 的所有可能性；
- 遍历 a 与 b 的所有可能性并拼接，即可得到所有长度为 `2 * n` 的括号序列。

为了节省计算时间，我们在每次 `generate(i)` 函数返回之前，把返回值存储起来，下次再调用 `generate(i)` 时可以直接返回，不需要再递归计算。

```java
class Solution {
    ArrayList[] cache = new ArrayList[100];
    public List<String> generate(int n) {
        if (cache[n] != null) {
            return cache[n];
        }
        ArrayList<String> ans = new ArrayList();
        if (n == 0) {
            ans.add("");
        } else {
            for (int c = 0; c < n; ++c)
                for (String left: generate(c)) // 生成 c 个括号放在 a 的位置
                    for (String right: generate(n - 1 - c)) // -1 是因为下面添加了一对括号了。
                        ans.add("(" + left + ")" + right);
        }
        cache[n] = ans;
        return ans;
    }
    public List<String> generateParenthesis(int n) {
        return generate(n);
    }
}
```

**复杂度分析**

- 时间复杂度：$O(\dfrac{4^n}{\sqrt{n}})$，该分析与 方法二 类似。
- 空间复杂度：$O(\dfrac{4^n}{\sqrt{n}})$，此方法除答案数组外，中间过程中会存储与答案数组同样数量级的临时数组，是我们所需要的空间复杂度。

这种方法没看明白呀。。。

> https://leetcode-cn.com/problems/generate-parentheses/solution/hui-su-suan-fa-by-liweiwei1419/

状态转移方程是：

```
dp[k] = "(" + dp[可能的括号对数] + ")" + dp[剩下的括号对数]
// 即
dp[k] = "(" + dp[j] + ")" + dp[k - j - 1] , j = 0, 1, ..., i - 1
```
