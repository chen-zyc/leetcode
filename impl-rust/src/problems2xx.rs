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
