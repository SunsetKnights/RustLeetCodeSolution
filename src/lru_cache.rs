use std::collections::HashMap;
pub struct LRUCache {
    val: Vec<(i32, i32)>,
    link: Vec<(usize, usize)>, // 双向循环链表
    head: usize,
    key_locate: HashMap<i32, usize>,
    len: i32,
}
/**
 * 146. LRU 缓存
 * 请你设计并实现一个满足  LRU (最近最少使用) 缓存 约束的数据结构。
 * 实现 LRUCache 类：
 *     LRUCache(int capacity) 以 正整数 作为容量 capacity 初始化 LRU 缓存
 *     int get(int key) 如果关键字 key 存在于缓存中，则返回关键字的值，否则返回 -1 。
 *     void put(int key, int value) 如果关键字 key 已经存在，则变更其数据值 value ；
 *     如果不存在，则向缓存中插入该组 key-value 。
 *     如果插入操作导致关键字数量超过 capacity ，则应该 逐出 最久未使用的关键字。
 * 函数 get 和 put 必须以 O(1) 的平均时间复杂度运行。
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            val: Vec::with_capacity(capacity as usize),
            link: Vec::with_capacity(capacity as usize),
            head: 0,
            key_locate: HashMap::with_capacity(capacity as usize),
            len: 0,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&idx) = self.key_locate.get(&key) {
            if idx != self.head {
                let (front, next) = self.link[idx];
                self.link[front].1 = next;
                self.link[next].0 = front;
                let (front, _) = self.link[self.head];
                self.link[idx].0 = front;
                self.link[front].1 = idx;
                self.link[idx].1 = self.head;
                self.link[self.head].0 = idx;
                self.head = idx;
            }
            self.val[self.head].1
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&idx) = self.key_locate.get(&key) {
            if idx != self.head {
                let (front, next) = self.link[idx];
                self.link[front].1 = next;
                self.link[next].0 = front;
                let (front, _) = self.link[self.head];
                self.link[idx].0 = front;
                self.link[front].1 = idx;
                self.link[idx].1 = self.head;
                self.link[self.head].0 = idx;
                self.head = idx;
            }
            self.val[self.head].1 = value;
        } else {
            if self.len as usize == self.link.capacity() {
                let (front, _) = self.link[self.head];
                self.head = front;
                self.key_locate.remove(&self.val[self.head].0);
                self.key_locate.insert(key, self.head);
                self.val[self.head] = (key, value);
            } else {
                self.val.push((key, value));
                self.link.push((0, 0));
                let (front, _) = self.link[self.head];
                *self.link.last_mut().unwrap() = (front, self.head);
                self.link[front].1 = self.len as usize;
                self.link[self.head].0 = self.len as usize;
                self.head = self.len as usize;
                self.key_locate.insert(key, self.len as usize);
                self.len += 1;
            }
        }
    }
}
