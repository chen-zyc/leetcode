- [117. 填充每个节点的下一个右侧节点指针 II](#117-填充每个节点的下一个右侧节点指针-ii)
  - [官方题解](#官方题解)
    - [方法一：层次遍历](#方法一层次遍历)
    - [方法二：使用已建立的 next 指针](#方法二使用已建立的-next-指针)

------------------------------

# 117. 填充每个节点的下一个右侧节点指针 II

给定一个二叉树

```c
struct Node {
  int val;
  Node *left;
  Node *right;
  Node *next;
}
```

填充它的每个 next 指针，让这个指针指向其下一个右侧节点。如果找不到下一个右侧节点，则将 next 指针设置为 NULL。

初始状态下，所有 next 指针都被设置为 NULL。


进阶：

- 你只能使用常量级额外空间。
- 使用递归解题也符合要求，本题中递归程序占用的栈空间不算做额外的空间复杂度。
 

示例：

![](assets/no_0117_populating_next_right_pointers_in_each_node_ii1.png)

```
输入：root = [1,2,3,4,5,null,7]
输出：[1,#,2,3,#,4,5,7,#]
解释：给定二叉树如图 A 所示，你的函数应该填充它的每个 next 指针，以指向其下一个右侧节点，如图 B 所示。
```
 

提示：

- 树中的节点数小于 6000
- -100 <= node.val <= 100

--------------------

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/populating-next-right-pointers-in-each-node-ii
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/populating-next-right-pointers-in-each-node-ii/solution/tian-chong-mei-ge-jie-dian-de-xia-yi-ge-you-ce-15/

### 方法一：层次遍历

这道题希望我们把二叉树各个层的点组织成链表，一个非常直观的思路是层次遍历。树的层次遍历基于广度优先搜索，它按照层的顺序遍历二叉树，在遍历第 i 层前，一定会遍历完第 i - 1 层。

算法如下：初始化一个队列 q，将根结点放入队列中。当队列不为空的时候，记录当前队列大小为 n，从队列中以此取出 n 个元素并通过这 n 个元素拓展新节点。如此循环，直到队列为空。我们不难写出这样的代码：

```c++
q.push(root);
while(!q.empty()) {
    int n = q.size();
    for (int i = 1; i <= n; ++i) {
        auto f = q.front();
        q.pop();
        // 拓展新节点
        if (f->left) {
            q.push(f->left);
        }
        if (f->right) {
            q.push(f->right);
        }
        // ... 遍历当前取出的 f
    }
}
```

这样做可以保证每次遍历的 n 个点都是同一层的。我们可以在遍历每一层的时候修改这一层节点的 next 指针，这样就可以把每一层都组织成链表。

```go
func connect(root *Node) *Node {
    if root == nil {
        return nil
    }
    q := []*Node{root}
    for len(q) > 0 {
        tmp := q
        q = nil
        for i, node := range tmp {
            if i+1 < len(tmp) {
                node.Next = tmp[i+1]
            }
            if node.Left != nil {
                q = append(q, node.Left)
            }
            if node.Right != nil {
                q = append(q, node.Right)
            }
        }
    }
    return root
}
```

**复杂度分析**

记树上的点的个数为 N。

- 时间复杂度：$O(N)$。我们需要遍历这棵树上所有的点，时间复杂度为 $O(N)$。
- 空间复杂度：$O(N)$。即队列的空间代价。


### 方法二：使用已建立的 next 指针

因为必须处理树上的所有节点，所以无法降低时间复杂度，但是可以尝试降低空间复杂度。

在方法一中，因为对树的结构一无所知，所以使用队列保证有序访问同一层的所有节点，并建立它们之间的连接。然而不难发现：一旦在某层的节点之间建立了 $\rm next$ 指针，那这层节点实际上形成了一个链表。因此，如果先去建立某一层的 $\rm next$ 指针，再去遍历这一层，就无需再使用队列了。

基于该想法，提出降低空间复杂度的思路：如果第 i 层节点之间已经建立 $\rm next$ 指针，就可以通过 $\rm next$ 指针访问该层的所有节点，同时对于每个第 i 层的节点，我们又可以通过它的 $\rm left$ 和 $\rm right$ 指针知道其第 i+1 层的孩子节点是什么，所以遍历过程中就能够按顺序为第 i + 1 层节点建立 $\rm next$ 指针。

具体来说：

- 从根节点开始。因为第 0 层只有一个节点，不需要处理。可以在上一层为下一层建立 $\rm next$ 指针。该方法最重要的一点是：位于第 x 层时为第 x + 1 层建立 $\rm next$ 指针。一旦完成这些连接操作，移至第 x + 1 层为第 x + 2 层建立 $\rm next$ 指针。
- 当遍历到某层节点时，该层节点的 $\rm next$ 指针已经建立。这样就不需要队列从而节省空间。每次只要知道下一层的最左边的节点，就可以从该节点开始，像遍历链表一样遍历该层的所有节点。

```go
func connect(root *Node) *Node {
    start := root
    for start != nil {
        var nextStart, last *Node
        handle := func(cur *Node) {
            if cur == nil {
                return
            }
            // 这一层第一个节点
            if nextStart == nil {
                nextStart = cur
            }
            if last != nil {
                last.Next = cur
            }
            last = cur
        }
        for p := start; p != nil; p = p.Next {
            handle(p.Left)
            handle(p.Right)
        }
        start = nextStart
    }
    return root
}
```

- 时间复杂度：$O(N)$。分析同「方法一」。
- 空间复杂度：$O(1)$。
