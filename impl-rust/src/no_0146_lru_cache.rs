use std::collections::HashMap;
use std::collections::LinkedList;

struct LRUCache {
    map: HashMap<i32, i32>,
    list: LinkedList<i32>,
    capacity: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * 
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        LRUCache{
            map: HashMap::with_capacity(capacity as usize),
            list: LinkedList::new(),
            capacity,
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        let res = self.map.get(&key);
        if res.is_some() {
            // 将 item 移到前面
            // todo: 这里怎么获得 item 的指针呢？
        }
        unimplemented!()
    }
    
    fn put(&self, key: i32, value: i32) {
        unimplemented!()
    }
}
