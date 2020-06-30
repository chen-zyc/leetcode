use std::collections::LinkedList;

struct CQueue {
    stack1: LinkedList<i32>, // 用来插入的
    stack2: LinkedList<i32>, // 用来删除的
    cur_is_stack2: bool,     // 元素现在是不是在第2个栈中
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        CQueue {
            stack1: LinkedList::new(),
            stack2: LinkedList::new(),
            cur_is_stack2: false,
        }
    }

    fn append_tail(&mut self, value: i32) {
        // if self.cur_is_stack2 {
        //     // 把整数挪到栈1
        //     self.move_to_stack1();
        // }
        self.stack1.push_front(value);
    }

    fn delete_head(&mut self) -> i32 {
        // if !self.cur_is_stack2 {
        //     // 把整数挪到栈2
        //     self.move_to_stack2();
        // }
        if self.stack2.is_empty() {
            self.move_to_stack2();
        }
        self.stack2.pop_front().unwrap_or(-1)
    }

    fn move_to_stack1(&mut self) {
        while let Some(value) = self.stack2.pop_front() {
            self.stack1.push_front(value);
        }
        self.cur_is_stack2 = false;
    }

    fn move_to_stack2(&mut self) {
        while let Some(value) = self.stack1.pop_front() {
            self.stack2.push_front(value);
        }
        self.cur_is_stack2 = true;
    }
}

// https://leetcode-cn.com/problems/yong-liang-ge-zhan-shi-xian-dui-lie-lcof/solution/jian-zhi-offer-09-yong-liang-ge-zhan-shi-xian-du-5/
// 用数组来模拟栈。
#[derive(Default)]
struct CQueue2 {
    stack1: Vec<i32>, // 用来插入的
    stack2: Vec<i32>, // 用来删除的
}

impl CQueue2 {
    fn new() -> Self {
        Default::default()
    }

    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.stack2.is_empty() {
            // 还可以这样：先反转，再 append
            // self.stack1.reverse();
            // self.stack2.append(&mut self.stack1);
            while let Some(value) = self.stack1.pop() {
                self.stack2.push(value);
            }
        }
        self.stack2.pop().unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cqueue_example1() {
        let mut q = CQueue::new();
        q.append_tail(3);
        assert_eq!(q.delete_head(), 3);
        assert_eq!(q.delete_head(), -1);
    }

    #[test]
    fn test_cqueue_example2() {
        let mut q = CQueue::new();
        assert_eq!(q.delete_head(), -1);
        q.append_tail(5);
        q.append_tail(2);
        assert_eq!(q.delete_head(), 5);
        assert_eq!(q.delete_head(), 2);
    }

    #[test]
    fn test_cqueue2_example1() {
        let mut q = CQueue2::new();
        q.append_tail(3);
        assert_eq!(q.delete_head(), 3);
        assert_eq!(q.delete_head(), -1);
    }

    #[test]
    fn test_cqueue2_example2() {
        let mut q = CQueue2::new();
        assert_eq!(q.delete_head(), -1);
        q.append_tail(5);
        q.append_tail(2);
        assert_eq!(q.delete_head(), 5);
        assert_eq!(q.delete_head(), 2);
    }
}
