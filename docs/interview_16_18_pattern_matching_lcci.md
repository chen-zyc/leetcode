- [面试题 16.18. 模式匹配](#面试题-1618-模式匹配)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：枚举](#方法一枚举)

------------------------------

# 面试题 16.18. 模式匹配

## 题目

你有两个字符串，即 pattern 和 value。 pattern 字符串由字母"a"和"b"组成，用于描述字符串中的模式。例如，字符串"catcatgocatgo"匹配模式"aabab"（其中"cat"是"a"，"go"是"b"），该字符串也匹配像"a"、"ab"和"b"这样的模式。但需注意"a"和"b"不能同时表示相同的字符串。编写一个方法判断value字符串是否匹配pattern字符串。

示例 1：

```
输入： pattern = "abba", value = "dogcatcatdog"
输出： true
```

示例 2：

```
输入： pattern = "abba", value = "dogcatcatfish"
输出： false
```

示例 3：

```
输入： pattern = "aaaa", value = "dogcatcatdog"
输出： false
```

示例 4：

```
输入： pattern = "abba", value = "dogdogdogdog"
输出： true
解释： "a"="dogdog",b=""，反之也符合规则
```

提示：

- 0 <= len(pattern) <= 1000
- 0 <= len(value) <= 1000
- 你可以假设pattern只包含字母"a"和"b"，value仅包含小写字母。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/pattern-matching-lcci
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/pattern-matching-lcci/solution/mo-shi-pi-pei-by-leetcode-solution/

**前言**

本题的算法实现不难，但是细节较多。题目中给出的 pattern 和 value 的长度可以为 0，因此需要充分考虑边界情况。

### 方法一：枚举

**思路与算法**

我们设 pattern 的长度为 $\ell_p$​，value 的长度为 $\ell_v$​。根据题目描述，我们需要给字母 a 和 b 分配不同的字符串值（可以为空字符串），使得将 pattern 中的字母替换成对应的字符串后，结果与 value 相同。

在分配字符串之前，我们不妨**先分配 a 和 b 对应字符串的长度**。如果确定了长度，那么我们只要将 value 按照 pattern 中出现字母的顺序，划分成 $\ell_p$​ 个子串，并判断其中 a 对应的子串是否相同，以及 b 对应的子串是否相同即可。具体地，假设 pattern 中出现了 $c_a$​ 个 a 以及 $\ell_p - c_a$​ 个 b，并且 a 和 b 对应字符串的长度分别为 $\ell_a$​ 和 $\ell_b$​，那么必须要满足：

$$
c_a * \ell_a + (\ell_p - c_a) * \ell_b = \ell_v
$$

其中 $c_a$​ 是已知的常量，$\ell_a$​ 和 $\ell_b$​ 是未知数。这是一个二元一次方程，可能无解、有唯一解或者无数解。然而我们需要的仅仅是自然数解，也就是 $\ell_a$​ 和 $\ell_b$​ 都大于等于 0 的解，因此我们可以**直接枚举 $\ell_a$​ 的值**，它必须是 $[0, \frac{\ell_v}{c_a}]$ 之间的自然数，否则 $\ell_b$​ 就不会大于等于 0 了。在枚举 $\ell_a$​ 之后，我们将其带入等式并解出 $\ell_b$​。如果 $\ell_b$​ 是整数，我们就枚举了一组 a 和 b 的可能长度。

在枚举了长度之后，我们就可以根据 pattern 来将 value 划分成 $\ell_p$​ 个子串。具体地，我们遍历 pattern，并用一个指针 pos 来帮助我们进行划分。当我们遍历到一个 a 时，我们取出从 pos 开始，长度为 $\ell_a$​ 的子串。如果这是我们第一次遇到字母 a，我们就得到了 a 应该对应的子串；否则，我们将取出的子串与 a 应该对应的子串进行比较，如果不相同，说明模式匹配失败。

同理，当我们遍历到一个 b 时，我们取出从 pos 开始，长度为 $\ell_b$​ 的子串，根据是否第一次遇到字母 b 来进行比较。在比较结束后，我们将 pos 向后移动，进行下一个字母的匹配。

在遍历完成之后，如果匹配没有失败，我们还需要判断一下 a 和 b 是否对应了不同的子串。只有它们对应的子串不同时，才是一种满足题目要求的模式匹配。

**细节**

上面的算法看上去不是很复杂：我们只需要用一重循环枚举 $\ell_a$​，计算出 $\ell_b$​，再用一重循环遍历 pattern 以及移动 pos 即可。但就像我们在「前言」部分所说的，本题有非常多的细节需要考虑。

我们回到二元一次方程：

$$
c_a * \ell_a + (\ell_p - c_a) * \ell_b = \ell_v
$$

如果我们枚举 $\ell_a$​，那么**必须要求 $c_a \neq 0$**，因为在 $c_a = 0$ 的情况下，原方程如果有解，那么一定有无数解（因为 $\ell_a$​ 可以取任意值）。因此如果 $c_a = 0$，我们就必须枚举 $\ell_b$​。这无疑增加了编码的复杂度，因为需要根据 $c_a$​ 的值选择对 $\ell_a$​ 或 $\ell_b$​ 进行枚举，失去了统一性。并且，如果 $\ell_p - c_a$​ 也为 0，那么我们连 $\ell_b$​ 都无法枚举。

因此，我们必须梳理一下判断的逻辑：

1. 如果 pattern 为空，那么只有在 value 也为空时，它们才能匹配；
2. 如果 value 为空，那么如果 pattern 也为空，就和第一条的情况相同；如果 pattern 中只出现了一种字母，我们可以令该字母为空，另一没有出现的字母为任意非空串，就可以正确匹配；如果 pattern 中出现了两种字母，那么就无法正确匹配，因为这两种字母都必须为空串，而题目描述中规定它们不能表示相同的字符串；
3. 如果 pattern 和 value 均非空，那么我们需要枚举 pattern 中出现的那个字母（如果两个字母均出现，可以枚举任意一个）对应的长度，使用上面提到的算法进行判断。

对于上面的第三条，我们可以根据「对称性」减少代码的编写的复杂度：我们还是固定枚举 $\ell_a$​，但如果 $c_a < \ell_p - c_a$​，即 a 出现的次数少于 b 出现的次数(比如 a 出现了 0 次)，那么我们就将 pattern 中所有的 a 替换成 b，b 替换成 a。这样做就**保证了 a 出现了至少一次**（$c_a > 0$），枚举 $\ell_a$​ 就不会有任何问题，同时不会影响答案的正确性。

这样一来，我们就可以优化判断的逻辑：

1. 我们首先保证 pattern 中 a 出现的次数不少于 b 出现的次数。如果不满足，我们就将 a 和 b 互相替换；
2. 如果 value 为空，那么要求 pattern 也为空（$\ell_p = 0$）或者只出现了字母 a（$\ell_p - c_a = 0$），这两种情况均等同于 $\ell_p - c_a = 0$。在其余情况下，都无法匹配成功；
3. 如果 pattern 为空且 value 不为空，那么无法匹配成功；
4. 如果 pattern 和 value 均非空，我们就可以枚举 $\ell_a$​ 并使用上面提到的算法进行判断。

下面给出的代码遵循了这样的逻辑。读者也可以尝试自己归纳出一套逻辑并编写代码。

```go
func patternMatching(pattern string, value string) bool {
    countA, countB := 0, 0
    for i := 0; i < len(pattern); i++ {
        if pattern[i] == 'a' {
            countA++
        } else {
            countB++
        }
    }
    if countA < countB {
        countA, countB = countB, countA
        tmp := ""
        for i := 0; i < len(pattern); i++ {
            if pattern[i] == 'a' {
                tmp += "b"
            } else {
                tmp += "a"
            }
        }
        pattern = tmp
    }
    if len(value) == 0 {
        return countB == 0
    }
    // 此时 value 不为空
    if len(pattern) == 0 {
        return false
    }

    // 现在 value 和 pattern 都不为空了。

    for lenA := 0; countA * lenA <= len(value); lenA++ {
        rest := len(value) - countA * lenA // B 占的所有的字节数
        if (countB == 0 && rest == 0) || (countB != 0 && rest % countB == 0) { // 保证一个 B 占的字节数是整数
            var lenB int // 一个 B 占的字节数
            if countB == 0 {
                lenB = 0
            } else {
                lenB = rest / countB
            }
            pos, correct := 0, true
            var valueA, valueB string
            for i := 0; i < len(pattern); i++ {
                if pattern[i] == 'a' {
                    sub := value[pos:pos+lenA]
                    if len(valueA) == 0 {
                        valueA = sub
                    } else if valueA != sub {
                        correct = false
                        break
                    }
                    pos += lenA
                } else {
                    sub := value[pos:pos+lenB] // 如果 lenB == 0 呢？那就一直给 valueB 赋值
                    if len(valueB) == 0 {
                        valueB = sub
                    } else if valueB != sub {
                        correct = false
                        break
                    }
                    pos += lenB
                }
            }
            if correct && valueA != valueB {
                return true
            }
        } 
    }
    return false
}
```

复杂度分析

本题的时空复杂度不易分析（因为涉及到二元一次方程解的个数），这里只近似地给出一个结果。

- 时间复杂度：$O(l_v^2)$，其中 $\ell_p$​ 和 $\ell_v$​ 分别是 pattern 和 value 的长度。由于 $\ell_a$​ 必须是 $[0, \frac{\ell_v}{c_a}]$ 中的自然数，并且 $\frac{1}{2}l_p \leq c_a \leq l_p$​，因此方程解的个数为 $O(\frac{l_v}{l_p})$。对于每一组解，我们需要 $O(l_p + l_v)$ 的时间来进行判断，因此总时间复杂度为 $O((l_p + l_v)\frac{l_v}{l_p})$。根据大 O 表示法的定义（渐进上界），可以看成 $O(l_v^2)$。
- 空间复杂度：$O(l_v)$。我们需要存储 a 和 b 对应的子串，它们的长度之和不会超过 $l_v$​。
