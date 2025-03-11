use lru_cache::LRUCache;

mod lru_cache;
fn main() {
    let mut cache = LRUCache::new(2);
    cache.put(2, 1);
    cache.put(3, 2);
    cache.get(3);
    cache.get(2);
    cache.put(4, 3);
    cache.get(2);
    cache.get(3);
    cache.get(4);
}
