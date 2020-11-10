- [49. 字母异位词分组](#49-字母异位词分组)
  - [官方题解](#官方题解)
    - [方法一：排序数组分类](#方法一排序数组分类)
    - [方法二：按计数分类](#方法二按计数分类)

------------------------------

# 49. 字母异位词分组

给定一个字符串数组，将字母异位词组合在一起。字母异位词指字母相同，但排列不同的字符串。

示例:

```
输入: ["eat", "tea", "tan", "ate", "nat", "bat"]
输出:
[
  ["ate","eat","tea"],
  ["nat","tan"],
  ["bat"]
]
```

说明：

- 所有输入均为小写字母。
- 不考虑答案输出的顺序。

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/group-anagrams
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/group-anagrams/solution/zi-mu-yi-wei-ci-fen-zu-by-leetcode/

### 方法一：排序数组分类

当且仅当它们的排序字符串相等时，两个字符串是字母异位词。

维护一个映射 `ans : {String -> List}`，其中每个键 `K` 是一个排序字符串，每个值是初始输入的字符串列表，排序后等于 `K`。

```java
class Solution {
    public List<List<String>> groupAnagrams(String[] strs) {
        if (strs.length == 0) return new ArrayList();
        Map<String, List> ans = new HashMap<String, List>();
        for (String s : strs) {
            char[] ca = s.toCharArray();
            Arrays.sort(ca);
            String key = String.valueOf(ca);
            if (!ans.containsKey(key)) ans.put(key, new ArrayList());
            ans.get(key).add(s);
        }
        return new ArrayList(ans.values());
    }
}
```

- 时间复杂度：$O(NK \log K)$，其中 N 是 strs 的长度，而 K 是 strs 中字符串的最大长度。当我们遍历每个字符串时，外部循环具有的复杂度为 $O(N)$。然后，我们在 $O(K \log K)$ 的时间内对每个字符串排序。
- 空间复杂度：$O(NK)$，排序存储在 ans 中的全部信息内容。


### 方法二：按计数分类
当且仅当它们的字符计数（每个字符的出现次数）相同时，两个字符串是字母异位词。

我们可以将每个字符串 `s` 转换为字符数 `count`，由26个非负整数组成，表示 `a`，`b`, `c` 的数量等。我们使用这些计数作为哈希映射的基础。

```py
class Solution:
    def groupAnagrams(strs):
        ans = collections.defaultdict(list)
        for s in strs:
            count = [0] * 26
            for c in s:
                count[ord(c) - ord('a')] += 1
            ans[tuple(count)].append(s)
        return ans.values()
```

- 时间复杂度：$O(NK)$，其中 N 是 strs 的长度，而 K 是 strs 中字符串的最大长度。计算每个字符串的字符串大小是线性的，我们统计每个字符串。
- 空间复杂度：$O(NK)$，排序存储在 ans 中的全部信息内容。
