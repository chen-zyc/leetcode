
#  Z 字形变换

## 方法一：按行排序

**思路**: 通过从左向右迭代字符串，我们可以轻松地确定字符位于 Z 字形图案中的哪一行。

> 图形中的每一行都用一个列表来保存。

**算法**

我们可以使用 $\text{min}( \text{numRows}, \text{len}(s))$ 个列表来表示 Z 字形图案中的非空行。

从左到右迭代 s，将每个字符添加到合适的行。可以使用当前行和当前方向这两个变量对合适的行进行跟踪。

只有当我们向上移动到最上面的行或向下移动到最下面的行时，当前方向才会发生改变。

```Java
class Solution {
    public String convert(String s, int numRows) {

        if (numRows == 1) return s;

        List<StringBuilder> rows = new ArrayList<>();
        for (int i = 0; i < Math.min(numRows, s.length()); i++)
            rows.add(new StringBuilder());

        int curRow = 0;
        boolean goingDown = false;

        for (char c : s.toCharArray()) {
            rows.get(curRow).append(c);
            if (curRow == 0 || curRow == numRows - 1) goingDown = !goingDown;
            curRow += goingDown ? 1 : -1;
        }

        StringBuilder ret = new StringBuilder();
        for (StringBuilder row : rows) ret.append(row);
        return ret.toString();
    }
}
```

**复杂度分析**

时间复杂度：$O(n)$，其中 $n == \text{len}(s)$.

空间复杂度：$O(n)$



## 方法二：按行访问

**思路**

按照与逐行读取 Z 字形图案相同的顺序访问字符串。

**算法**

首先访问 行 0 中的所有字符(这里的行是指最终图形中的行)，接着访问 行 1，然后 行 2，依此类推...

对于所有整数 k，

- 行 0 中的字符位于索引 $k \; (2 \cdot \text{numRows} - 2)$ 处;
- 行 numRows−1 中的字符位于索引 $k \; (2 \cdot \text{numRows} - 2) + \text{numRows} - 1$ 处;
- 内部的 行 i 中的字符位于索引 $k \; (2 \cdot \text{numRows}-2)+i$ 以及 $(k+1)(2 \cdot \text{numRows}-2)- i$ 处;

```java
class Solution {
    public String convert(String s, int numRows) {

        if (numRows == 1) return s;

        StringBuilder ret = new StringBuilder();
        int n = s.length();
        int cycleLen = 2 * numRows - 2;

        for (int i = 0; i < numRows; i++) {
            for (int j = 0; j + i < n; j += cycleLen) {
                ret.append(s.charAt(j + i));
                // 不是第一行和最后一行，添加第二个字符。
                if (i != 0 && i != numRows - 1 && j + cycleLen - i < n)
                    ret.append(s.charAt(j + cycleLen - i));
            }
        }
        return ret.toString();
    }
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 $n == \text{len}(s)$。每个索引被访问一次。
- 空间复杂度：$O(n)$。对于 C++ 实现，如果返回字符串不被视为额外空间，则复杂度为 $O(1)$。


**我的分析**

上图中，我们把 A-E 作为一个循环，A-D 的长度是 numRows，E-F 比 A-D 少了两个字符，长度是 numRows-2。所以一个循环的长度是 $2 \times numRows-2$。为了后面方便计算，我们把一个循环的长度用 Len 来表示：$Len = 2 \times numRows-2$。

第一行中，A 毫无疑问是 `s[0]`，G 和 A 相差一个循环的长度，所以应该把 `s[Len]` 放在这里。M 和 A 相差两个循环的长度，所以长度应该是 `s[2*Len]`。第一行后面的位置以此类推。

第二行。在一个循环中，第二行有两个字符。我们以 H 和 L 为例。H 可以认为是 G 后面第一个字符，而 G 是 `s[Len]`，所以应该是把 `s[Len+1]` 放在 H 的位置。同理，L 是 M 的上一个字符，M 是 `s[2*Len]`，所以应该把 `s[2*Len-1]` 放在 L 的位置。

第三行和第二行是类似的，同样有两个字符。

最后一行只有一个字符。最后一个字符和所有循环中第一个字符相差 numRows-1 个字符，所以 D 和 A 相差 numRows-1 个字符，A 是 `s[0]`，D 应该是 `s[0+numRows-1]`。J 和 G 也是相差 numRows-1 个字符，G 是 `s[Len]`，那么 J 就是 `s[Len+numRows-1]`。