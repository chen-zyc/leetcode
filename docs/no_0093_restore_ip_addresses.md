- [93. 复原IP地址](#93-复原ip地址)
  - [题目](#题目)
  - [题解](#题解)
    - [方法一：递归](#方法一递归)

------------------------------

# 93. 复原IP地址

## 题目

给定一个只包含数字的字符串，复原它并返回所有可能的 IP 地址格式。

有效的 IP 地址正好由四个整数（每个整数位于 0 到 255 之间组成），整数之间用 '.' 分隔。

示例:

```
输入: "25525511135"
输出: ["255.255.11.135", "255.255.111.35"]
```

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/restore-ip-addresses
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

## 题解

> 链接：https://leetcode-cn.com/problems/restore-ip-addresses/solution/fu-yuan-ipdi-zhi-by-leetcode-solution/

### 方法一：递归

**思路与算法**

由于我们需要找出所有可能复原出的 IP 地址，因此可以考虑使用递归的方法，对所有可能的字符串分隔方式进行搜索，并筛选出满足要求的作为答案。

设题目中给出的字符串为 $s$。我们用递归函数 $\textit{dfs}(\textit{segId}, \textit{segStart})$ 表示我们正在从 $s[\textit{segStart}]$ 的位置开始，搜索 IP 地址中的第 $\textit{segId}$ 段，其中 $\textit{segId} \in \{0, 1, 2, 3\}$。由于 IP 地址的每一段必须是 $[0, 255]$ 中的整数，因此我们从 $\textit{segStart}$ 开始，从小到大依次枚举当前这一段 IP 地址的结束位置 $\textit{segEnd}$。如果满足要求，就递归地进行下一段搜索，调用递归函数 $\textit{dfs}(\textit{segId} + 1, \textit{segEnd} + 1)$。

特别地，由于 IP 地址的每一段不能有前导零，因此如果 $s[\textit{segStart}]$ 等于字符 $0$，那么 IP 地址的第 $\textit{segId}$ 段只能为 $0$，需要作为特殊情况进行考虑。

在递归搜索的过程中，如果我们已经得到了全部的 $4$ 段 IP 地址（即 $\textit{segId} = 4$），并且遍历完了整个字符串（即 $\textit{segStart} = |s|$，其中 $|s|$ 表示字符串 $s$ 的长度），那么就复原出了一种满足题目要求的 IP 地址，我们将其加入答案。在其它的时刻，如果提前遍历完了整个字符串，那么我们需要结束搜索，回溯到上一步。

```go
const SEG_COUNT = 4

var (
    ans []string
    segments []int
)

func restoreIpAddresses(s string) []string {
    segments = make([]int, SEG_COUNT)
    ans = []string{}
    dfs(s, 0, 0)
    return ans
}

func dfs(s string, segId, segStart int) {
    // 如果找到了 4 段 IP 地址并且遍历完了字符串，那么就是一种答案
    if segId == SEG_COUNT {
        if segStart == len(s) {
            ipAddr := ""
            for i := 0; i < SEG_COUNT; i++ {
                ipAddr += strconv.Itoa(segments[i])
                if i != SEG_COUNT - 1 {
                    ipAddr += "."
                }
            }
            ans = append(ans, ipAddr)
        }
        return
    }

    // 如果还没有找到 4 段 IP 地址就已经遍历完了字符串，那么提前回溯
    if segStart == len(s) {
        return
    }
    // 由于不能有前导零，如果当前数字为 0，那么这一段 IP 地址只能为 0
    if s[segStart] == '0' {
        segments[segId] = 0
        dfs(s, segId + 1, segStart + 1)
    }
    // 一般情况，枚举每一种可能性并递归
    addr := 0
    for segEnd := segStart; segEnd < len(s); segEnd++ {
        addr = addr * 10 + int(s[segEnd] - '0')
        if addr > 0 && addr <= 0xFF {
            segments[segId] = addr
            dfs(s, segId + 1, segEnd + 1)
        } else {
            break
        }
    }
}
```

**复杂度分析**

我们用 $\text{SEG\_COUNT} = 4$ 表示 IP 地址的段数。

- 时间复杂度：$O(3^\text{SEG\_COUNT} * |s|)$。由于 IP 地址的每一段的位数不会超过 3，因此在递归的每一层，我们最多只会深入到下一层的 3 种情况。由于 $\text{SEG\_COUNT} = 4$，对应着递归的最大层数，所以递归本身的时间复杂度为 $O(3^\text{SEG\_COUNT})$。如果我们复原出了一种满足题目要求的 IP 地址，那么需要 $O(|s|)$ 的时间将其加入答案数组中，因此总时间复杂度为 $O(3^\text{SEG\_COUNT} * |s|)$。 
- 空间复杂度：$O(\text{SEG\_COUNT})$，这里只计入除了用来存储答案数组以外的额外空间复杂度。递归使用的空间与递归的最大深度 $\text{SEG\_COUNT}$ 成正比。并且在上面的代码中，我们只额外使用了长度为 $\text{SEG\_COUNT}$ 的数组 $\textit{segments}$ 存储已经搜索过的 IP 地址，因此空间复杂度为 $O(\text{SEG\_COUNT})$。
