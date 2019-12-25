# generate_parentheses

Given *n* pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

For example, given *n* = 3, a solution set is:

```
[
  "((()))",
  "(()())",
  "(())()",
  "()(())",
  "()()()"
]
```



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



