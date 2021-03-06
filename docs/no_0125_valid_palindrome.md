- [125. 验证回文串](#125-验证回文串)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：筛选 + 判断](#方法一筛选--判断)
    - [方法二：在原字符串上直接判断](#方法二在原字符串上直接判断)

------------------------------

# 125. 验证回文串

## 题目

给定一个字符串，验证它是否是回文串，只考虑字母和数字字符，可以忽略字母的大小写。

说明：本题中，我们将空字符串定义为有效的回文串。

示例 1:

```
输入: "A man, a plan, a canal: Panama"
输出: true
```

示例 2:

```
输入: "race a car"
输出: false
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/valid-palindrome
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 题解

链接：https://leetcode-cn.com/problems/valid-palindrome/solution/yan-zheng-hui-wen-chuan-by-leetcode-solution/

**前言**

本题考查的是语言中常用字符（串）相关 API 的使用。本题解会给出解题思路以及参考代码，如果代码中有读者不熟悉的 API，可以自行查阅资料学习。


### 方法一：筛选 + 判断

最简单的方法是对字符串 s 进行一次遍历，并将其中的字母和数字字符进行保留，放在另一个字符串 sgood 中。这样我们只需要判断 sgood 是否是一个普通的回文串即可。

判断的方法有两种。第一种是使用语言中的字符串翻转 API 得到 sgood 的逆序字符串 `sgood_rev`，只要这两个字符串相同，那么 sgood 就是回文串。

```java
class Solution {
    public boolean isPalindrome(String s) {
        StringBuffer sgood = new StringBuffer();
        int length = s.length();
        for (int i = 0; i < length; i++) {
            char ch = s.charAt(i);
            if (Character.isLetterOrDigit(ch)) {
                sgood.append(Character.toLowerCase(ch));
            }
        }
        StringBuffer sgood_rev = new StringBuffer(sgood).reverse();
        return sgood.toString().equals(sgood_rev.toString());
    }
}
```

第二种是使用双指针。初始时，左右指针分别指向 sgood 的两侧，随后我们不断地将这两个指针相向移动，每次移动一步，并判断这两个指针指向的字符是否相同。当这两个指针相遇时，就说明 sgood 是回文串。

```go
func isPalindrome(s string) bool {
    var sgood string
    for i := 0; i < len(s); i++ {
        if isalnum(s[i]) {
            sgood += string(s[i])
        }
    }

    n := len(sgood)
    sgood = strings.ToLower(sgood)
    for i := 0; i < n/2; i++ {
        if sgood[i] != sgood[n-1-i] {
            return false
        }
    }
    return true
}

func isalnum(ch byte) bool {
    return (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9')
}
```

复杂度分析

- 时间复杂度：$O(|s|)$，其中 $|s|$ 是字符串 s 的长度。
- 空间复杂度：$O(|s|)$。由于我们需要将所有的字母和数字字符存放在另一个字符串中，在最坏情况下，新的字符串 sgood 与原字符串 s 完全相同，因此需要使用 $O(|s|)$ 的空间。

### 方法二：在原字符串上直接判断

我们可以对方法一中第二种判断回文串的方法进行优化，就可以得到只使用 $O(1)$ 空间的算法。

我们直接在原字符串 s 上使用双指针。在移动任意一个指针时，需要不断地向另一指针的方向移动，直到遇到一个字母或数字字符，或者两指针重合为止。也就是说，我们每次将指针移到下一个字母字符或数字字符，再判断这两个指针指向的字符是否相同。

```go
func isPalindrome(s string) bool {
    s = strings.ToLower(s)
    left, right := 0, len(s) - 1
    for left < right {
        for left < right && !isalnum(s[left]) {
            left++
        }
        for left < right && !isalnum(s[right]) {
            right--
        }
        if left < right {
            if s[left] != s[right] {
                return false
            }
            left++
            right--
        }
    }
    return true
}

func isalnum(ch byte) bool {
    return (ch >= 'A' && ch <= 'Z') || (ch >= 'a' && ch <= 'z') || (ch >= '0' && ch <= '9')
}
```

复杂度分析

- 时间复杂度：$O(|s|)$，其中 $|s|$ 是字符串 s 的长度。
- 空间复杂度：$O(1)$。
