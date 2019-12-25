# multiply_strings

Given two non-negative integers `num1` and `num2` represented as strings, return the product of `num1` and `num2`.

**Note:**

1. The length of both `num1` and `num2` is < 110.
2. Both `num1` and `num2` contains only digits `0-9`.
3. Both `num1` and `num2` does not contain any leading zero.
4. You **must not use any built-in BigInteger library** or **convert the inputs to integer** directly.


两个字符串相乘，不能用内建的BigInteger或者将字符串转换成整型。



![](pic/multiply_strings.jpg)

A:123的索引分别是0..2，B:45的索引分别是0..1，每个乘积都是按照两位来算的，不足的补零。

`A[0]*B[0]`放在了result[0..1], `A[1]*B[0]`放在了result[1..2] ... `A[1]*B[1]`放在了result[2..3] ...

所以规律就是 `A[i] * B[j]` 被放在了result[i+j.. i+j+1]。