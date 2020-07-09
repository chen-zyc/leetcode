- [面试题 17.13. 恢复空格](#面试题-1713-恢复空格)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：Trie + 动态规划](#方法一trie--动态规划)
    - [方法二：字符串哈希](#方法二字符串哈希)

------------------------------

# 面试题 17.13. 恢复空格

## 题目

哦，不！你不小心把一个长篇文章中的空格、标点都删掉了，并且大写也弄成了小写。像句子"I reset the computer. It still didn’t boot!"已经变成了"iresetthecomputeritstilldidntboot"。在处理标点符号和大小写之前，你得先把它断成词语。当然了，你有一本厚厚的词典dictionary，不过，**有些词没在词典里**。假设文章用sentence表示，设计一个算法，把文章断开，要求**未识别的字符最少**，返回未识别的字符数。

注意：本题相对原题稍作改动，只需返回未识别的字符数
 

示例：

```
输入：
dictionary = ["looked","just","like","her","brother"]
sentence = "jesslookedjustliketimherbrother"
输出： 7
解释： 断句后为"jess looked just like tim her brother"，共7个未识别字符。
```

提示：

- 0 <= len(sentence) <= 1000
- dictionary中总字符数不超过 150000。
- 你可以认为dictionary和sentence中只包含小写字母。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/re-space-lcci
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

## 题解

看答案前先尝试着解一下：
1. 从字典中可以得出单词的最小(min)和最大(max)长度。
2. 每次从句子中截取 min 个字母：
   1. 如果在字典中，那么就可以在它前后断句了。
   2. 如果不在字典中，那么就增加一个字母（直到 max 长度），再尝试。
   3. 如果 max 长度的还不在字典中，那么可以确定字典中没有以它为前缀的，可以去掉一个字母，再尝试。

看有没有在字典中可以使用前缀树。

那怎么解决"未识别的字符最少呢"？不知道啊。。。还是看答案吧。

> 链接：https://leetcode-cn.com/problems/re-space-lcci/solution/hui-fu-kong-ge-by-leetcode-solution/

### 方法一：Trie + 动态规划

**预备知识**

- 字典树 Trie

**思路和算法**

定义 `dp[i]` 表示考虑前 i 个字符最少的未识别的字符数量，从前往后计算 `dp` 值。

考虑转移方程，每次转移的时候我们考虑第 `j(j <= i)` 个到第 `i` 个字符组成的子串 `sentence[j−1⋯i−1]` （注意字符串下标从 0 开始）是否能在词典中找到，如果能找到的话按照定义转移方程即为

$$
\textit{dp}[i]=\min(\textit{dp}[i],\textit{dp}[j-1])
$$

> 能找到的话，前 i 个字符未识别的字符数(`dp[i]`) 就是前 j-1 个字符中未识别的字符数了(`dp[j-1]`)。

否则没有找到的话我们可以复用 `dp[i−1]` 的状态再加上当前未被识别的第 i 个字符，因此此时 `dp` 值为

$$
\textit{dp}[i]=dp[i-1]+1
$$

最后问题化简成了转移的时候如何快速判断当前子串是否存在于词典中，与「[单词拆分](https://leetcode-cn.com/problems/word-break/)」类似我们可以选择用哈希表来优化，但笔者实测下来速度很慢，因为用哈希表来实现本身有两个问题，一个是哈希表本身的常数很大，还有一个是我们在枚举子串是否在词典中的时候有些其实是没有必要的枚举。简单举例，如果我们有词典：`['aabc', 'babc', 'cbc']`，但是我们在**倒序枚举**的时候检查 `dc` 这个子串没出现在词典中以后我们就没必要再接着往前枚举是否有合法的子串了，因为 `dc` 本身已经不是词典中「任意一个单词的后缀」，我们再接着枚举 `*dc` 或者 `**dc` 判断其是否在词典中都是无用功。

因此最终笔者选择了用字典树 Trie 来优化查找，Trie 是一种最大程度利用多个字符串前缀信息的数据结构，它可以在 $O(w)$ 的时间复杂度内判断一个字符串是否是一个字符串集合中某个字符串的前缀，其中 w 代表字符串的长度。这里具体实现不再展开，我们只讲怎么使用。上文提到了哈希表实现的时候会出现很多冗余的判断，最关键的一点就是当前枚举的子串已经不再是词典中「任意一个单词的后缀」，这点我们可以利用 Trie 来解决。

我们将词典中所有的单词「**反序**」插入字典树中，然后每次转移的时候我们从当前的下标 i 出发倒序遍历 $i-1,i-2,\cdots,0$。在 Trie 上从根节点出发开始走，直到走到当前的字符 `sentence[j]` 在 Trie 上没有相应的位置，说明 `sentence[j⋯i−1]` 不存在在词典中，且它已经不是「任意一个单词的后缀」，此时我们直接跳出循环即可。否则，我们需要判断当前的子串是否是一个单词，这里我们直接在插入 Trie 的时候在单词末尾的节点打上一个 isEnd 的标记即可，这样我们在走到某个节点的时候就可以判断是否是一个单词的末尾并根据状态转移方程更新我们的 `dp` 值。

具体实现以及示例的图画解析可以看下面：

![](assets/interview_17_13_re_space_lcci.gif)

```go
func respace(dictionary []string, sentence string) int {
    n, inf := len(sentence), 0x3f3f3f3f // inf 是最大值也行
    root := &Trie{next: [26]*Trie{}}
    for _, word := range dictionary {
        root.insert(word)
    }
    dp := make([]int, n + 1)
    for i := 1; i < len(dp); i++ {
        dp[i] = inf
    }
    for i := 1; i <= n; i++ { // i 表示前 i 个字符
        dp[i] = dp[i-1] + 1
        curPos := root
        for j := i; j >= 1; j-- { // j 遍历的字符范围是 sentence[0..=j-1]，并且是倒序遍历的。
            t := int(sentence[j-1] - 'a')
            if curPos.next[t] == nil {
                // 如果没有以 t 为后缀的，说明 t 不在字典中，那么 dp[i] = dp[i-1] + 1.
                break
            } else if curPos.next[t].isEnd {
                // sentence[j-1 ..= i-1] 在字典中（isEnd 说明 t 是某个单词的第一个字母）
                dp[i] = min(dp[i], dp[j-1])
            }
            if dp[i] == 0 {
                // 前 i 个都在字典中，j 就不需要再向前遍历了
                break
            }
            // 匹配下一个节点
            curPos = curPos.next[t]
        }
    }
    return dp[n]
}

type Trie struct {
    next [26]*Trie
    isEnd bool
}

func (this *Trie) insert(s string) {
    curPos := this
    for i := len(s) - 1; i >= 0; i-- {
        t := int(s[i] - 'a')
        if curPos.next[t] == nil {
            curPos.next[t] = &Trie{next: [26]*Trie{}}
        }
        curPos = curPos.next[t]
    }
    curPos.isEnd = true
}

func min(x, y int) int {
    if x < y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度：$O(|\textit{dictionary}|+n^2)$，其中 $|\textit{dictionary}|$ 代表词典中的总字符数，`n=sentence.length`。建字典树的时间复杂度取决于单词的总字符数，即 $|\textit{dictionary}|$，因此时间复杂度为 $O(|\textit{dictionary}|)$。`dp` 数组一共有 n+1 个状态，每个状态转移的时候最坏需要 $O(n)$ 的时间复杂度，因此时间复杂度为 $O(n^2)$。
- 空间复杂度：$O(|\textit{dictionary}|*S+n)$，其中 S 代表字符集大小，这里为小写字母数，因此 `S=26`。我们可以这样考虑空间复杂度的渐进上界：对于字典树而言，如果节点个数为 $|node|$，字符集大小为 S，那么空间代价为 $O(|node|*S)$；因为这里的节点数一定小于词典中的总字符数，故 $O(|node|*S) = O(|dictionary|*S)$。`dp` 数组的空间代价为 $O(n)$。


> 如果是用前缀来判断会怎么样？
> 假设一个单词在字典中，那么就会把每个前缀都匹配一遍，如果发现没到单词的结束，就只能当作没存在。
> 这样会导致很多不必要的匹配。


### 方法二：字符串哈希

**预备知识**

- 字符串哈希：可参考「1392. 最长快乐前缀」官方题解中的「背景知识」。

**思路和算法**

我们使用字典树的目的是查找某一个串 s 是否在一个串的集合 S 当中，并且当我们知道 s 是否在 S 中之后，可以快速的知道在 s 后添加某一个新的字母得到的新串 s' 是否在 S 中，这个转移的过程是 $O(1)$ 的。这是我们采用字典树而放弃使用 HashMap 类容器的一个理由，这些容器不能实现 s 到 s' 的 $O(1)$ 转移，但字典树可以。

其实还用一种字符串哈希的方法也能实现 $O(1)$ 的转移，就是「预备知识」中提到的 Rabin-Karp 方法。我们用这种方法替换字典树，时间复杂度不变，空间复杂度可以优化到 $O(n + q)$，其中 n 为 sentence 中元素的个数，q 为词典中单词的个数。

代码如下。

```go
const (
    P = math.MaxInt32
    BASE = 41
)

func respace(dictionary []string, sentence string) int {
    hashValues := map[int]bool{}
    for _, word := range dictionary {
        hashValues[getHash(word)] = true
    }
    f := make([]int, len(sentence) + 1)
    for i := 1; i < len(f); i++ {
        f[i] = len(sentence)
    }
    for i := 1; i <= len(sentence); i++ {
        f[i] = f[i-1] + 1
        hashValue := 0
        for j := i; j >= 1; j-- {
            t := int(sentence[j-1] - 'a') + 1
            hashValue = (hashValue * BASE + t) % P
            if hashValues[hashValue] {
                f[i] = min(f[i], f[j-1])
            }
        }
    }
    return f[len(sentence)]
}

func getHash(s string) int {
    hashValue := 0
    for i := len(s) - 1; i >= 0; i-- {
        hashValue = (hashValue * BASE + int(s[i] - 'a') + 1) % P
    }
    return hashValue
}

func min(x, y int) int {
    if x < y {
        return x
    }
    return y
}
```

**复杂度分析**

- 时间复杂度：$O(|\textit{dictionary}|+n^2)$，同方法一。
- 空间复杂度：$O(n + q)$，其中 n 为 sentence 中元素的个数，q 为词典中单词的个数。

