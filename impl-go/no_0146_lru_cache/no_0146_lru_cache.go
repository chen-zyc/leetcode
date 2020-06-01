package no_0146_lru_cache

import (
	"container/list"
)

type Item struct {
	key   int
	value int
}

type LRUCache struct {
	m        map[int]*list.Element
	l        *list.List
	capacity int
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		m:        make(map[int]*list.Element, capacity),
		l:        list.New(),
		capacity: capacity,
	}
}

func (this *LRUCache) Get(key int) int {
	ele := this.m[key]
	if ele == nil {
		return -1
	}
	this.l.MoveToFront(ele)
	return ele.Value.(*Item).value
}

func (this *LRUCache) Put(key int, value int) {
	ele := this.m[key]
	if ele != nil {
		ele.Value.(*Item).value = value
		this.l.MoveToFront(ele)
		return
	}
	for len(this.m) >= this.capacity {
		back := this.l.Back()
		this.l.Remove(back)
		delete(this.m, back.Value.(*Item).key)
	}
	ele = this.l.PushFront(&Item{key: key, value: value})
	this.m[key] = ele
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */
