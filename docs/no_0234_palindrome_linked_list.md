- [234. 回文链表](#234-回文链表)
  - [官方题解](#官方题解)
    - [方法一：将值复制到数组中后用双指针法](#方法一将值复制到数组中后用双指针法)
    - [方法二：递归](#方法二递归)
    - [方法三：快慢指针](#方法三快慢指针)

------------------------------

# 234. 回文链表

请判断一个链表是否为回文链表。

示例 1:

```
输入: 1->2
输出: false
```

示例 2:

```
输入: 1->2->2->1
输出: true
```

进阶： 你能否用 $O(n)$ 时间复杂度和 $O(1)$ 空间复杂度解决此题？

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/palindrome-linked-list
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。

## 官方题解

> 链接：https://leetcode-cn.com/problems/palindrome-linked-list/solution/hui-wen-lian-biao-by-leetcode-solution/

### 方法一：将值复制到数组中后用双指针法

确定数组列表是否是回文很简单，我们可以使用双指针法来比较两端的元素，并向中间移动。一个指针从起点向中间移动，另一个指针从终点向中间移动。这需要 $O(n)$ 的时间，因为访问每个元素的时间是 $O(1)$，而有 n 个元素要访问。

```go
func isPalindrome(head *ListNode) bool {
    vals := []int{}
    for ; head != nil; head = head.Next {
        vals = append(vals, head.Val)
    }
    n := len(vals)
    for i, v := range vals[:n/2] {
        if v != vals[n-1-i] {
            return false
        }
    }
    return true
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 指的是链表的元素个数。
    - 第一步： 遍历链表并将值复制到数组中，$O(n)$。
    - 第二步：双指针判断是否为回文，执行了 $O(n/2)$ 次的判断，即 $O(n)$。
    - 总的时间复杂度：$O(2n) = O(n)$。
- 空间复杂度：$O(n)$，其中 n 指的是链表的元素个数，我们使用了一个数组列表存放链表的元素值。


### 方法二：递归

```go
func isPalindrome(head *ListNode) bool {
    frontPointer := head
    var recursivelyCheck func(*ListNode) bool
    recursivelyCheck = func(curNode *ListNode) bool {
        if curNode != nil {
            // 一直调用到 curNode.Next 是空的
            if !recursivelyCheck(curNode.Next) {
                return false
            }
            // 第一次比较执行这里时，curNode 是最后一个节点，frontPointer 是第一个节点。
            if curNode.Val != frontPointer.Val {
                return false
            }
            // frontPointer 后移，同时函数返回后，curNode 相当于前移了。
            frontPointer = frontPointer.Next
        }
        return true
    }
    return recursivelyCheck(head)
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 指的是链表的大小。
- 空间复杂度：$O(n)$，其中 n 指的是链表的大小。我们要理解计算机如何运行递归函数，在一个函数中调用一个函数时，计算机需要在进入被调用函数之前跟踪它在当前函数中的位置（以及任何局部变量的值），通过运行时存放在堆栈中来实现（堆栈帧）。在堆栈中存放好了数据后就可以进入被调用的函数。在完成被调用函数之后，他会弹出堆栈顶部元素，以恢复在进行函数调用之前所在的函数。在进行回文检查之前，递归函数将在堆栈中创建 n 个堆栈帧，计算机会逐个弹出进行处理。所以在使用递归时空间复杂度要考虑堆栈的使用情况。

这种方法不仅使用了 $O(n)$ 的空间，且比第一种方法更差，因为在许多语言中，堆栈帧的开销很大（如 Python），并且最大的运行时堆栈深度为 1000（可以增加，但是有可能导致底层解释程序内存出错）。为每个节点创建堆栈帧极大的限制了算法能够处理的最大链表大小。


### 方法三：快慢指针

避免使用 $O(n)$ 额外空间的方法就是改变输入。

我们可以将链表的后半部分反转（修改链表结构），然后将前半部分和后半部分进行比较。比较完成后我们应该将链表恢复原样。虽然不需要恢复也能通过测试用例，但是使用该函数的人通常不希望链表结构被更改。

该方法虽然可以将空间复杂度降到 $O(1)$，但是在并发环境下，该方法也有缺点。在并发环境下，函数运行时需要锁定其他线程或进程对链表的访问，因为在函数执行过程中链表会被修改。

算法

整个流程可以分为以下五个步骤：

1. 找到前半部分链表的尾节点。
2. 反转后半部分链表。
3. 判断是否回文。
4. 恢复链表。
5. 返回结果。

执行步骤一，我们可以计算链表节点的数量，然后遍历链表找到前半部分的尾节点。

我们也可以使用快慢指针在一次遍历中找到：慢指针一次走一步，快指针一次走两步，快慢指针同时出发。当快指针移动到链表的末尾时，慢指针恰好到链表的中间。通过慢指针将链表分为两部分。

**若链表有奇数个节点，则中间的节点应该看作是前半部分**。(多出来的那个中间节点没有参与比较)

步骤二可以使用「206. 反转链表」问题中的解决方法来反转链表的后半部分。

步骤三比较两个部分的值，当后半部分到达末尾则比较完成，可以忽略计数情况中的中间节点。

步骤四与步骤二使用的函数相同，再反转一次恢复链表本身。

```go
func reverseList(head *ListNode) *ListNode {
    var prev, cur *ListNode = nil, head
    for cur != nil {
        nextTmp := cur.Next
        cur.Next = prev
        prev = cur
        cur = nextTmp
    }
    return prev
}

func endOfFirstHalf(head *ListNode) *ListNode {
    fast := head
    slow := head
    for fast.Next != nil && fast.Next.Next != nil {
        fast = fast.Next.Next
        slow = slow.Next
    }
    return slow
}

func isPalindrome(head *ListNode) bool {
    if head == nil {
        return true
    }

    // 找到前半部分链表的尾节点并反转后半部分链表
    firstHalfEnd := endOfFirstHalf(head)
    secondHalfStart := reverseList(firstHalfEnd.Next)

    // 判断是否回文
    p1 := head
    p2 := secondHalfStart
    result := true
    for result && p2 != nil {
        if p1.Val != p2.Val {
            result = false
        }
        p1 = p1.Next
        p2 = p2.Next
    }

    // 还原链表并返回结果
    firstHalfEnd.Next = reverseList(secondHalfStart)
    return result
}
```

复杂度分析

- 时间复杂度：$O(n)$，其中 n 指的是链表的大小。
- 空间复杂度：$O(1)$。我们只会修改原本链表中节点的指向，而在堆栈上的堆栈帧不超过 $O(1)$。


> 评论中有人提到，可以在找中间节点时就把前面的部分反转了。