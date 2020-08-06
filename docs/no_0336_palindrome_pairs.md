- [回文对](#回文对)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：枚举前缀和后缀](#方法一枚举前缀和后缀)
    - [方法二：字典树 + manacher](#方法二字典树--manacher)

------------------------------

# 回文对

## 题目

给定一组**唯一**的单词， 找出所有**不同**的索引对 `(i, j)`，使得列表中的两个单词， `words[i] + words[j]` ，可拼接成回文串。

示例 1:

```
输入: ["abcd","dcba","lls","s","sssll"]
输出: [[0,1],[1,0],[3,2],[2,4]] 
解释: 可拼接成的回文串为 ["dcbaabcd","abcddcba","slls","llssssll"]
```

示例 2:

```
输入: ["bat","tab","cat"]
输出: [[0,1],[1,0]] 
解释: 可拼接成的回文串为 ["battab","tabbat"]
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/palindrome-pairs
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

> 链接：https://leetcode-cn.com/problems/palindrome-pairs/solution/hui-wen-dui-by-leetcode-solution/

**写在前面**

本题可以想到暴力做法，我们枚举每一对字符串的组合，暴力判断它们是否能够构成回文串即可。时间复杂度 $O(n^2\times m)$，其中 $n$ 是字符串的数量，$m$ 是字符串的平均长度。时间复杂度并不理想，考虑进行优化。

### 方法一：枚举前缀和后缀

**思路及算法**

假设存在两个字符串 $s_1$ 和 $s_2$ ，$s_1+s_2$ 是一个回文串，记这两个字符串的长度分别为 $len_1$ 和 $len_2$，我们分三种情况进行讨论：

1. $\textit{len}_1 = \textit{len}_2$，这种情况下 $s_1$ 是 $s_2$ 的翻转。
2. $\textit{len}_1 > \textit{len}_2$，这种情况下我们可以将 $s_1$ 拆成左右两部分：$t_1$ 和 $t_2$ ，其中 $t_1$ 是 $s_2$ 的翻转，$t_2$ 是一个回文串。
3. $\textit{len}_1 < \textit{len}_2$，这种情况下我们可以将 $s_2$ 拆成左右两部分：$t_1$ 和 $t_2$ ，其中 $t_2$ 是 $s_1$ 的翻转，$t_1$ 是一个回文串。

这样，对于每一个字符串，我们令其为 $s_1$ 和 $s_2$ 中较长的那一个，然后找到可能和它构成回文串的字符串即可。

具体地说，我们枚举每一个字符串 $k$，令其为 $s_1$ 和 $s_2$ 中较长的那一个，那么 $k$ 可以被拆分为两部分，$t_1$ 和 $t_2$ 。 

1. 当 $t_1$ 是回文串时，符合情况 $3$，我们只需要查询给定的字符串序列中是否包含 $t_2$ 的翻转。
2. 当 $t_2$ 是回文串时，符合情况 $2$，我们只需要查询给定的字符串序列中是否包含 $t_1$ 的翻转。

也就是说，我们要枚举字符串 $k$ 的每一个前缀和后缀，判断其是否为回文串。如果是回文串，我们就查询其剩余部分的翻转是否在给定的字符串序列中出现即可。

![https://leetcode-cn.com/problems/palindrome-pairs/solution/c-zi-dian-shu-zhu-shi-xiang-xi-by-li-zhi-chao-4/](assets/no_0336_palindrome_pairs.jpg)

注意到空串也是回文串，所以我们可以将 $k$ 拆解为 $k+\varnothing$ 或 $\varnothing+k$，这样我们就能将情况 $1$ 也解释为特殊的情况 $2$ 或情况 $3$。

而要实现这些操作，我们只需要设计一个能够在一系列字符串中查询「某个字符串的子串的翻转」是否存在的数据结构，有两种实现方法：

- 我们可以使用字典树存储所有的字符串。在进行查询时，我们将待查询串的子串逆序地在字典树上进行遍历，即可判断其是否存在。
- 我们可以使用哈希表存储所有字符串的翻转串。在进行查询时，我们判断带查询串的子串是否在哈希表中出现，就等价于判断了其翻转是否存在。

代码

下面给出的是使用字典树的代码：

```go
type Node struct {
    ch [26]int
    flag int
}

var tree []Node

func palindromePairs(words []string) [][]int {
    tree = []Node{Node{[26]int{}, -1}} // tree[0] 是 root
    n := len(words)
    for i := 0; i < n; i++ {
        insert(words[i], i)
    }
    ret := [][]int{}
    for i := 0; i < n; i++ {
        word := words[i]
        m := len(word)
        for j := 0; j <= m; j++ {
            if isPalindrome(word, j, m - 1) { // word[j, m-1] 是回文
                leftId := findWord(word, 0, j - 1) // 在树中找 word[0, j-1] 的翻转
                if leftId != -1 && leftId != i {
                    ret = append(ret, []int{i, leftId})
                }
            }
            if j != 0 && isPalindrome(word, 0, j - 1) { // word[0, j-1] 是回文
                rightId := findWord(word, j, m - 1) // 在树中找 word[j, m-1] 的翻转
                if rightId != -1 && rightId != i {
                    ret = append(ret, []int{rightId, i})
                }
            }
        }
    }
    return ret
}

func insert(s string, id int) {
    add := 0 // 树高
    for i := 0; i < len(s); i++ {
        x := int(s[i] - 'a')
        if tree[add].ch[x] == 0 {
            // 这里没有用链表，而是把所有的节点都放在一个切片里，用索引来构成链接关系。
            tree = append(tree, Node{[26]int{}, -1})
            tree[add].ch[x] = len(tree) - 1
        }
        add = tree[add].ch[x]
    }
    tree[add].flag = id
}

func findWord(s string, left, right int) int {
    add := 0
    for i := right; i >= left; i-- {
        x := int(s[i] - 'a')
        if tree[add].ch[x] == 0 {
            return -1
        }
        add = tree[add].ch[x]
    }
    return tree[add].flag
}

func isPalindrome(s string, left, right int) bool {
    for i := 0; i < (right - left + 1) / 2; i++ {
        if s[left + i] != s[right - i] {
            return false
        }
    }
    return true
}
```

下面给出的是使用哈希表的代码：

```go
func palindromePairs(words []string) [][]int {
    wordsRev := []string{}
    indices := map[string]int{}

    n := len(words)
    // 把单词的翻转放到字典里。wordsRev 没有必要吧？
    for _, word := range words {
        wordsRev = append(wordsRev, reverse(word))
    }
    for i := 0; i < n; i++ {
        indices[wordsRev[i]] = i
    }

    ret := [][]int{}
    for i := 0; i < n; i++ {
        word := words[i]
        m := len(word)
        if m == 0 {
            continue
        }
        for j := 0; j <= m; j++ {
            if isPalindrome(word, j, m - 1) {
                leftId := findWord(word, 0, j - 1, indices)
                if leftId != -1 && leftId != i {
                    ret = append(ret, []int{i, leftId})
                }
            }
            if j != 0 && isPalindrome(word, 0, j - 1) {
                rightId := findWord(word, j, m - 1, indices)
                if rightId != -1 && rightId != i {
                    ret = append(ret, []int{rightId, i})
                }
            }
        }
    }
    return ret
}

func findWord(s string, left, right int, indices map[string]int) int {
    if v, ok := indices[s[left:right+1]]; ok {
        return v
    }
    return -1
}

func isPalindrome(s string, left, right int) bool {
    for i := 0; i < (right - left + 1) / 2; i++ {
        if s[left + i] != s[right - i] {
            return false
        }
    }
    return true
}

func reverse(s string) string {
    n := len(s)
    b := []byte(s)
    for i := 0; i < n/2; i++ {
        b[i], b[n-i-1] = b[n-i-1], b[i]
    }
    return string(b)
}
```


**复杂度分析**

- 时间复杂度：$O(n \times m^2)$，其中 $n$ 是字符串的数量，$m$ 是字符串的平均长度。对于每一个字符串，我们需要 $O(m^2)$ 地判断其所有前缀与后缀是否是回文串，并 $O(m^2)$ 地寻找其所有前缀与后缀是否在给定的字符串序列中出现。 
- 空间复杂度：$O(n \times m)$，其中 $n$ 是字符串的数量，$m$ 是字符串的平均长度。为字典树的空间开销。

### 方法二：字典树 + manacher

**说明**

方法二为竞赛难度，在面试中不作要求。学有余力的读者可以学习在字符串中寻找最长回文串的「manacher 算法」。

**思路及算法**

注意到方法一中，对于每一个字符串 $k$，我们需要 $O(m^2)$ 地判断 $k$ 的所有前缀与后缀是否是回文串，还需要 $O(m^2)$ 地判断 $k$ 的所有前缀与后缀是否在给定字符串序列中出现。我们可以优化这两部分的时间复杂度。

- 对于判断其所有前缀与后缀是否是回文串：
    - 利用 manacher 算法，可以线性地处理出每一个前后缀是否是回文串。
- 对于判断其所有前缀与后缀是否在给定的字符串序列中出现：
    - 对于给定的字符串序列，分别正向与反向建立字典树，利用正向建立的字典树验证 $k$ 的后缀的翻转，利用反向建立的字典树验证 $k$ 的前缀的翻转。

这样我们就可以快速找出能够和字符串 $k$ 构成回文串的字符串。

**注意**：因为该解法常数较大，因此在随机数据下的表现并没有方法一优秀。

```java
class Solution {
    public List<List<Integer>> palindromePairs(String[] words) {
        Trie trie1 = new Trie(); // 正向
        Trie trie2 = new Trie(); // 反向

        int n = words.length;
        for (int i = 0; i < n; i++) {
            trie1.insert(words[i], i);
            StringBuffer tmp = new StringBuffer(words[i]);
            tmp.reverse();
            trie2.insert(tmp.toString(), i);
        }

        List<List<Integer>> ret = new ArrayList<List<Integer>>();
        for (int i = 0; i < n; i++) {
            int[][] rec = manacher(words[i]);

            int[] id1 = trie2.query(words[i]);
            words[i] = new StringBuffer(words[i]).reverse().toString();
            int[] id2 = trie1.query(words[i]);

            int m = words[i].length();

            int allId = id1[m];
            if (allId != -1 && allId != i) {
                ret.add(Arrays.asList(i, allId));
            }
            for (int j = 0; j < m; j++) {
                if (rec[j][0] != 0) {
                    int leftId = id2[m - j - 1];
                    if (leftId != -1 && leftId != i) {
                        ret.add(Arrays.asList(leftId, i));
                    }
                }
                if (rec[j][1] != 0) {
                    int rightId = id1[j];
                    if (rightId != -1 && rightId != i) {
                        ret.add(Arrays.asList(i, rightId));
                    }
                }
            }
        }
        return ret;
    }

    public int[][] manacher(String s) {
        int n = s.length();
        // #a*b*c*d...!
        StringBuffer tmp = new StringBuffer("#");
        for (int i = 0; i < n; i++) {
            if (i > 0) {
                tmp.append('*');
            }
            tmp.append(s.charAt(i));
        }
        tmp.append('!');
        // 后面就看不懂了
        int m = n * 2;
        int[] len = new int[m];
        int[][] ret = new int[n][2];
        int p = 0, maxn = -1;
        for (int i = 1; i < m; i++) {
            len[i] = maxn >= i ? Math.min(len[2 * p - i], maxn - i) : 0;
            while (tmp.charAt(i - len[i] - 1) == tmp.charAt(i + len[i] + 1)) {
                len[i]++;
            }
            if (i + len[i] > maxn) {
                p = i;
                maxn = i + len[i];
            }
            if (i - len[i] == 1) {
                ret[(i + len[i]) / 2][0] = 1;
            }
            if (i + len[i] == m - 1) {
                ret[(i - len[i]) / 2][1] = 1;
            }
        }
        return ret;
    }
}

class Trie {
    class Node {
        int[] ch = new int[26];
        int flag;

        public Node() {
            flag = -1;
        }
    }

    List<Node> tree = new ArrayList<Node>();

    public Trie() {
        tree.add(new Node());
    }

    public void insert(String s, int id) {
        int len = s.length(), add = 0;
        for (int i = 0; i < len; i++) {
            int x = s.charAt(i) - 'a';
            if (tree.get(add).ch[x] == 0) {
                tree.add(new Node());
                tree.get(add).ch[x] = tree.size() - 1;
            }
            add = tree.get(add).ch[x];
        }
        tree.get(add).flag = id;
    }

    public int[] query(String s) {
        int len = s.length(), add = 0;
        int[] ret = new int[len + 1];
        Arrays.fill(ret, -1);
        for (int i = 0; i < len; i++) {
            ret[i] = tree.get(add).flag;
            int x = s.charAt(i) - 'a';
            if (tree.get(add).ch[x] == 0) {
                return ret;
            }
            add = tree.get(add).ch[x];
        }
        ret[len] = tree.get(add).flag;
        return ret;
    }
}
```

**复杂度分析**

- 时间复杂度：$O(n \times m)$，其中 $n$ 是字符串的数量，$m$ 是字符串的平均长度。对于每一个字符串，我们需要 $O(m)$ 地判断其所有前缀与后缀是否是回文串，并 $O(m)$ 地寻找其所有前缀与后缀是否在给定的字符串序列中出现。
- 空间复杂度：$O(n \times m)$，其中 $n$ 是字符串的数量，$m$ 是字符串的平均长度。为字典树的空间开销。



