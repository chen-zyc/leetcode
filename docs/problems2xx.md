- [232. 用栈实现队列](#232-用栈实现队列)

------------------------------

# 232. 用栈实现队列

请你仅使用两个栈实现先入先出队列。队列应当支持一般队列的支持的所有操作（push、pop、peek、empty）：

实现 MyQueue 类：

- `void push(int x)` 将元素 x 推到队列的末尾
- `int pop()` 从队列的开头移除并返回元素
- `int peek()` 返回队列开头的元素
- `boolean empty()` 如果队列为空，返回 true ；否则，返回 false

说明：

- 你只能使用标准的栈操作 —— 也就是只有 push to top, peek/pop from top, size, 和 is empty 操作是合法的。
- 你所使用的语言也许不支持栈。你可以使用 list 或者 deque（双端队列）来模拟一个栈，只要是标准的栈操作即可。

进阶：

- 你能否实现每个操作均摊时间复杂度为 $O(1)$ 的队列？换句话说，执行 n 个操作的总时间复杂度为 $O(n)$，即使其中一个操作可能花费较长时间。

示例：

```
输入：
["MyQueue", "push", "push", "peek", "pop", "empty"]
[[], [1], [2], [], [], []]
输出：
[null, null, null, 1, 1, false]

解释：
MyQueue myQueue = new MyQueue();
myQueue.push(1); // queue is: [1]
myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
myQueue.peek(); // return 1
myQueue.pop(); // return 1, queue is [2]
myQueue.empty(); // return false
```

提示：

- 1 <= x <= 9
- 最多调用 100 次 push、pop、peek 和 empty
- 假设所有操作都是有效的 （例如，一个空的队列不会调用 pop 或者 peek 操作）

链接：https://leetcode-cn.com/problems/implement-queue-using-stacks

> 官方题解太简单了，这里是摘抄的[这个题解](https://leetcode-cn.com/problems/implement-queue-using-stacks/solution/yong-zhan-shi-xian-dui-lie-by-leetcode/)的

方法1：入队$O(n)$，出队$O(1)$

![](assets/0232_implement-queue-using-stacks1.png)

入队时：
1. 把 S1 倒到 S2。
2. 把 3 入栈。
3. 把 S2 的再倒回来。
4. 最后 S1 中的元素和入栈时的顺序正好相反。

![](assets/0232_implement-queue-using-stacks2.png)

出队时就简单了，直接从 S1 顶部弹出就行了。

方法2：入队$O(1)$, 出队$O(n)$

![](assets/0232_implement-queue-using-stacks3.png)

入队时直接压入 S1。

![](assets/0232_implement-queue-using-stacks4.png)

> 这个图应该是弄错了。

出队时：
1. 把 S1 的元素都导到 S2，这样 S2 的顶部就是最开始的那个元素。
2. 从 S2 的顶部把最开始的元素弹出。

整体复杂度是 $O(1)$，每个元素入栈两次（S1 和 S2 各一次），每个元素都出栈两次（S1 出一次，S2 出一次）。
