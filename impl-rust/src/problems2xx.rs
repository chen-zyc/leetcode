use crate::common::ListNode;

struct Solution;
impl Solution {
    /// 206. 反转链表
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 这个是从别人的答案中看到的，很小巧。

        // 反转之后的链表的第一个节点。
        let mut reserved = None;
        while let Some(mut node) = head {
            // head 指向下一个节点
            head = node.next;
            // 把 node 放到 reserved 后面
            node.next = reserved;
            reserved = Some(node);
        }
        reserved
    }
    pub fn reverse_list_1(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut ptr = &mut dummy;
        while let Some(mut node) = head {
            // 把 node 后面的节点赋值给 head，下一个循环用。
            let next = node.next.take();
            head = next;
            // 把 ptr 后面的节点赋值给 node.next。
            node.next = ptr.next.take();
            // 把 node 节点放到 ptr 的前面
            ptr.next = Some(node);
        }
        dummy.next
    }

    // 224. 基本计算器
    // https://leetcode-cn.com/problems/basic-calculator/
    pub fn calculate(s: String) -> i32 {
        let (mut sign, mut num, mut ans) = (1, 0, 0);
        let mut stack = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '+' => {
                    // 把 num 合到 ans 中
                    ans += num * sign;
                    num = 0;
                    sign = 1;
                }
                '-' => {
                    ans += num * sign;
                    num = 0;
                    sign = -1;
                }
                '(' => {
                    // 把前面的入栈
                    stack.push(ans);
                    stack.push(sign);
                    // 重新开始
                    ans = 0;
                    sign = 1;
                }
                ')' => {
                    // 把前面的出栈，并和当前的结果 ans 进行计算。
                    ans += sign * num; // 当前括号内的结果
                    ans = stack.pop().unwrap() * ans + stack.pop().unwrap();
                    sign = 1;
                    num = 0;
                }
                n if n.is_digit(10) => num = num * 10 + (n as u8 - '0' as u8) as i32,
                _ => {}
            }
        }
        ans += sign * num;

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!(Solution::calculate("1 + 1".to_owned()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_owned()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_owned()), 23);
    }

    #[test]
    fn test_reverse_list() {
        let head = ListNode::new_from_arr(&[1, 2, 3, 4, 5]);
        let expected = ListNode::new_from_arr(&[5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(head), expected);

        let head = ListNode::new_from_arr(&[1, 2]);
        let expected = ListNode::new_from_arr(&[2, 1]);
        assert_eq!(Solution::reverse_list(head), expected);

        assert_eq!(Solution::reverse_list(None), None);
    }
}

mod problem232 {
    use std::collections::LinkedList;

    struct MyQueue {
        // 新的元素在栈顶。
        stack_for_push: LinkedList<i32>,
        // 新的元素在栈底。
        stack_for_pop: LinkedList<i32>,
    }

    impl MyQueue {
        /** Initialize your data structure here. */
        fn new() -> Self {
            Self {
                stack_for_push: LinkedList::new(),
                stack_for_pop: LinkedList::new(),
            }
        }

        /** Push element x to the back of queue. */
        fn push(&mut self, x: i32) {
            self.stack_for_push.push_back(x);
        }

        /** Removes the element from in front of queue and returns that element. */
        fn pop(&mut self) -> i32 {
            self.move_to_stack_for_pop();
            self.stack_for_pop.pop_back().unwrap()
        }

        fn move_to_stack_for_pop(&mut self) {
            if self.stack_for_pop.is_empty() && !self.stack_for_push.is_empty() {
                while let Some(x) = self.stack_for_push.pop_back() {
                    self.stack_for_pop.push_back(x);
                }
            }
        }

        /** Get the front element. */
        fn peek(&mut self) -> i32 {
            self.move_to_stack_for_pop();
            self.stack_for_pop.back().unwrap().clone()
        }

        /** Returns whether the queue is empty. */
        fn empty(&self) -> bool {
            self.stack_for_push.is_empty() && self.stack_for_pop.is_empty()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_my_queue() {
            let mut q = MyQueue::new();
            q.push(1);
            q.push(2);
            assert_eq!(q.peek(), 1);
            q.pop();
            assert_eq!(q.empty(), false);
        }
    }
}
