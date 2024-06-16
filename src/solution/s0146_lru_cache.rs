use std::collections::{HashMap, VecDeque};

pub struct LRUCache {
    map: HashMap<i32, i32>,
    lru: VecDeque<i32>,
    capacity: i32,
    usage_count: HashMap<i32, usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            map: HashMap::new(),
            lru: VecDeque::with_capacity(capacity as usize),
            capacity,
            usage_count: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&value) = self.map.get(&key) {
            self.lru.push_back(key);
            *self.usage_count.entry(key).or_insert(0) += 1;
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(val) = self.map.get_mut(&key) {
            *val = value;
            self.lru.push_back(key);
            *self.usage_count.entry(key).or_insert(0) += 1;
            return;
        }

        if self.map.len() as i32 >= self.capacity {
            while let Some(evict) = self.lru.pop_front() {
                if let Some(count) = self.usage_count.get_mut(&evict) {
                    if *count == 1 {
                        self.map.remove(&evict);
                        self.usage_count.remove(&evict);
                        break;
                    } else {
                        *count -= 1;
                    }
                }
            }
        }

        self.map.insert(key, value);
        self.lru.push_back(key);
        self.usage_count.insert(key, 1);
    }
}
