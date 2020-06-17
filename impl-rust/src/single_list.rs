#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T>
where
    T: std::fmt::Debug,
{
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T>
where
    T: std::fmt::Debug,
{
    #[inline]
    fn new(val: T) -> Self {
        ListNode { val, next: None }
    }

    fn new_with_next(val: T, next: Option<Box<ListNode<T>>>) -> Self {
        ListNode { val, next }
    }
}

pub struct SingleList<T>
where
    T: std::fmt::Debug,
{
    head: Option<Box<ListNode<T>>>,
    len: usize,
}

impl<T> SingleList<T>
where
    T: std::fmt::Debug,
{
    fn new() -> Self {
        Self { head: None, len: 0 }
    }

    fn push_front(&mut self, val: T) {
        // let mut node = ListNode::new(val);
        // let next = self.head.take(); // 把整个链都返回
        // node.next = next;
        // self.head = Some(Box::new(node));
        // 更简洁的方式：
        let new_head = Some(Box::new(ListNode::new_with_next(val, self.head.take())));
        self.head = new_head;
        self.len += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.val
        })
    }

    fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    // 反转链表
    fn reverse(self) -> Self {
        let mut ptr = self.head;
        let mut new_list = SingleList::new();
        while let Some(mut node) = ptr {
            // 指向一个
            ptr = node.next.take();
            // 把 node 放到 new_list 中
            node.next = new_list.head;
            new_list.head = Some(node);
            new_list.len += 1;
        }
        new_list
    }
}

// 为 Clone 类型的实现
impl<T: Clone + std::fmt::Debug> SingleList<T> {
    // 反转链表，并且复制值。
    fn reverse_and_clone(&self) -> SingleList<T> {
        let mut new_list = SingleList::new();
        let mut next = self.head.as_ref().map(|node| &**node); // Option<&ListNode>
        while let Some(node) = next {
            new_list.push_front(node.val.clone());
            next = node.next.as_ref().map(|node| &**node);
        }

        new_list
    }
}

// 将 SingleList 转成 Vec<T>
impl<T> Into<Vec<T>> for SingleList<T>
where
    T: std::fmt::Debug,
{
    fn into(mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len);
        while let Some(v) = self.pop_front() {
            vec.push(v);
        }
        vec
    }
}

// 从 &[T] 构建 SingleList.
impl<'a, T> From<&'a [T]> for SingleList<T>
where
    T: std::fmt::Debug + Clone,
{
    fn from(item: &'a [T]) -> Self {
        let mut list = SingleList::new();
        for v in item {
            list.push_front(v.clone());
        }
        list
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_peek_pop_front() {
        let mut list = SingleList::new();
        list.push_front(1);
        list.push_front(2);

        assert_eq!(list.peek_front(), Some(&2));
        assert_eq!(list.peek_front(), Some(&2));

        let val = list.pop_front();
        assert_eq!(val, Some(2));
        assert_eq!(list.peek_front(), Some(&1));

        list.push_front(3);
        assert_eq!(list.peek_front(), Some(&3));

        let val = list.pop_front();
        assert_eq!(val, Some(3));
        assert_eq!(list.peek_front(), Some(&1));

        let val = list.pop_front();
        assert_eq!(val, Some(1));
        assert_eq!(list.peek_front(), None);

        let val = list.pop_front();
        assert_eq!(val, None);
    }

    #[test]
    fn test_reverse_and_clone() {
        // 2 -> 1
        let mut list = SingleList::new();
        list.push_front(1);
        list.push_front(2);

        let list = &list; // 不可变引用
        let mut new_list = list.reverse_and_clone(); // 1 -> 2
        assert_eq!(new_list.pop_front(), Some(1));
        assert_eq!(new_list.pop_front(), Some(2));
        assert_eq!(new_list.pop_front(), None);
    }

    #[test]
    fn test_reverse() {
        // b -> a
        let mut list = SingleList::new();
        list.push_front("a");
        list.push_front("b");

        // a -> b
        let mut new_list = list.reverse();
        assert_eq!(new_list.pop_front(), Some("a"));
        assert_eq!(new_list.pop_front(), Some("b"));
        assert_eq!(new_list.pop_front(), None);
    }

    #[test]
    fn test_into_vec() {
        // b -> a
        let mut list = SingleList::new();
        list.push_front("a");
        list.push_front("b");

        let vec: Vec<&str> = list.into();
        assert_eq!(vec, vec!["b", "a"]);
    }

    #[test]
    fn test_from_str_slice() {
        let vec = vec!["a", "b", "c"];
        let mut list = SingleList::from(&vec[1..]);
        assert_eq!(list.pop_front(), Some("c"));
        assert_eq!(list.pop_front(), Some("b"));
        assert_eq!(list.pop_front(), None);
    }
}
