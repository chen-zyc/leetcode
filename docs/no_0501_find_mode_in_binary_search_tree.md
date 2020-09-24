- [501. 二叉搜索树中的众数](#501-二叉搜索树中的众数)
  - [官方题解](#官方题解)
    - [方法一：Morris 中序遍历](#方法一morris-中序遍历)

------------------------------

# 501. 二叉搜索树中的众数

给定一个有相同值的二叉搜索树（BST），找出 BST 中的所有众数（出现频率最高的元素）。

假定 BST 有如下定义：

- 结点左子树中所含结点的值小于等于当前结点的值
- 结点右子树中所含结点的值大于等于当前结点的值
- 左子树和右子树都是二叉搜索树

例如： 给定 BST `[1,null,2,2]`,

```
   1
    \
     2
    /
   2
```

返回 `[2]`.

提示：如果众数超过1个，不需考虑输出顺序

进阶：你可以不使用额外的空间吗？（假设由递归产生的隐式调用栈的开销不被计算在内）

- 来源：力扣（LeetCode）
- 链接：https://leetcode-cn.com/problems/find-mode-in-binary-search-tree
- 著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。


## 官方题解

> 链接：https://leetcode-cn.com/problems/find-mode-in-binary-search-tree/solution/er-cha-sou-suo-shu-zhong-de-zhong-shu-by-leetcode-/

### 方法一：Morris 中序遍历

首先我们一定能想到一个最朴素的做法：因为这棵树的**中序遍历是一个有序的序列**，所以我们可以先获得这棵树的中序遍历，然后从扫描这个中序遍历序列，然后用一个哈希表来统计每个数字出现的个数，这样就可以找到出现次数最多的数字。但是这样做的空间复杂度显然不是 $O(1)$ 的，原因是哈希表和保存中序遍历序列的空间代价都是 $O(n)$。

首先，我们考虑在寻找出现次数最多的数时，不使用哈希表。 这个优化是基于 BST 中序遍历的性质：一棵 BST 的中序遍历序列是一个非递减的有序序列。例如：

```
      1
    /   \
   0     2
  / \    /
-1   0  2
```

这样一颗 BST 的中序遍历序列是 $\{ -1, 0, 0, 1, 2, 2 \}$。我们可以发现重复出现的数字一定是一个连续出现的，例如这里的 0 和 2，它们都重复出现了，并且所有的 0 都集中在一个连续的段内，所有的 2 也集中在一个连续的段内。我们可以顺序扫描中序遍历序列，用 $\rm base$ 记录当前的数字，用 $\rm count$ 记录当前数字重复的次数，用 $\rm maxCount$ 来维护已经扫描过的数当中出现最多的那个数字的出现次数，用 $\rm answer$ **数组**记录出现的众数。每次扫描到一个新的元素：

- 首先更新 $\rm base$ 和 $\rm count$:
    - 如果该元素和 $\rm base$ 相等，那么 $\rm count$ 自增 1
    - 否则将 $\rm base$ 更新为当前数字，$\rm count$ 复位为 1
- 然后更新 $\rm maxCount$：
    - 如果 $\rm count = maxCount$，那么说明当前的这个数字（$\rm base$）出现的次数等于当前众数出现的次数，将 $\rm base$ 加入 $\rm answer$ 数组
    - 如果 $\rm count > maxCount$，那么说明当前的这个数字（$\rm base$）出现的次数大于当前众数出现的次数，因此，我们需要将 $\rm maxCount$ 更新为 $\rm count$，**清空 $\rm answer$ 数组**后将 $\rm base$ 加入 $\rm answer$ 数组

我们可以把这个过程写成一个 `update` 函数。这样我们在寻找出现次数最多的数字的时候就可以省去一个哈希表带来的空间消耗。

然后，我们用 Morris 中序遍历的方法把中序遍历的空间复杂度优化到 $O(1)$。 我们在中序遍历的时候，一定先遍历左子树，然后遍历当前节点，最后遍历右子树。在常规方法中，我们用递归回溯或者是栈来保证遍历完左子树可以再回到当前节点，但这需要我们付出额外的空间代价。我们需要用一种巧妙地方法可以在 $O(1)$ 的空间下，遍历完左子树可以再回到当前节点。我们希望当前的节点在遍历完当前点的前驱之后被遍历，我们可以考虑修改它的前驱节点的 `right` 指针。当前节点的前驱节点的 `right` 指针可能本来就指向当前节点（前驱是当前节点的父节点），也可能是当前节点左子树最右下的节点。如果是后者，我们希望遍历完这个前驱节点之后再回到当前节点，可以将它的 `right` 指针指向当前节点。

Morris 中序遍历的一个重要步骤就是寻找当前节点的前驱节点，并且 Morris 中序遍历寻找下一个点始终是通过转移到 `right` 指针指向的位置来完成的。

- 如果当前节点没有左子树，则遍历这个点，然后跳转到当前节点的右子树。
- 如果当前节点有左子树，那么它的前驱节点一定在左子树上，我们可以在左子树上一直向右行走，找到当前点的前驱节点。
    - 如果前驱节点没有右子树，就将前驱节点的 `right` 指针指向当前节点。这一步是为了在遍历完前驱节点后能找到前驱节点的后继，也就是当前节点。
    - 如果前驱节点的右子树为当前节点，说明前驱节点已经被遍历过并被修改了 `right` 指针，这个时候我们重新将前驱的右孩子设置为空，遍历当前的点，然后跳转到当前节点的右子树。

因此我们可以得到这样的代码框架：

```c++
TreeNode *cur = root, *pre = nullptr;
while (cur) {
    if (!cur->left) {
        // ...遍历 cur
        cur = cur->right;
        continue;
    }
    pre = cur->left;
    while (pre->right && pre->right != cur) {
        pre = pre->right;
    }
    if (!pre->right) {
        pre->right = cur;
        cur = cur->left;
    } else {
        pre->right = nullptr;
        // ...遍历 cur
        cur = cur->right;
    }
}
```

最后我们将 `...遍历 cur` 替换成之前的 `update` 函数即可。

```go
func findMode(root *TreeNode) (answer []int) {
    var base, count, maxCount int
    update := func(x int) {
        if x == base {
            count++
        } else {
            base, count = x, 1
        }
        if count == maxCount {
            answer = append(answer, base)
        } else if count > maxCount {
            maxCount = count
            answer = []int{base}
        }
    }
    cur := root
    for cur != nil {
        if cur.Left == nil {
            update(cur.Val)
            cur = cur.Right
            continue
        }
        pre := cur.Left
        for pre.Right != nil && pre.Right != cur {
            pre = pre.Right
        }
        if pre.Right == nil {
            pre.Right = cur
            cur = cur.Left
        } else {
            pre.Right = nil
            update(cur.Val)
            cur = cur.Right
        }
    }
    return
}
```

复杂度分析

- 时间复杂度：$O(n)$。每个点被访问的次数不会超过两次，故这里的时间复杂度是 $O(n)$。
- 空间复杂度：$O(1)$。使用临时空间的大小和输入规模无关。
