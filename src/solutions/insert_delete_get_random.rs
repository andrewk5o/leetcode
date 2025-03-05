// 380. Insert Delete GetRandom O(1)
// https://leetcode.com/problems/insert-delete-getrandom-o1/

use rand::random_range;
use std::collections::HashMap;

pub struct RandomizedSet {
    hashmap: HashMap<i32, usize>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        RandomizedSet {
            hashmap: HashMap::new(),
            vec: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.hashmap.contains_key(&val) {
            false
        } else {
            self.hashmap.insert(val, self.vec.len());
            self.vec.push(val);
            true
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if !self.hashmap.contains_key(&val) {
            false
        } else {
            let idx = self.hashmap[&val];
            self.vec[idx] = self.vec[self.vec.len() - 1];
            self.hashmap.insert(self.vec[idx], idx);
            self.vec.pop();
            self.hashmap.remove(&val);
            true
        }
    }

    pub fn get_random(&self) -> i32 {
        self.vec[random_range(0..self.vec.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_randomized_set() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
        assert!(set.remove(1));
        assert!(!set.remove(1));
    }
}
