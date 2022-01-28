struct Bucket {
    pub items: Vec<(i32, i32)>,
}

impl Default for Bucket {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Bucket {
    fn get(&self, key: i32) -> i32 {
        for &pair in self.items.iter() {
            if pair.0 as i32 == key {
                return pair.1;
            }
        }
        return -1;
    }

    fn update(&mut self, key: i32, value: i32) {
        let mut found = false;
        for pair in self.items.iter_mut() {
            if pair.0 as i32 == key {
                pair.1 = value;
                found = true;
            }
        }
        if found == false {
            self.items.push((key, value))
        }
    }

    fn remove(&mut self, key: i32) {
        self.items.retain(|pair| pair.0 != key)
    }
}

struct MyHashMap {
    key_space: i32,
    hash_table: Vec<Bucket>,
}

impl MyHashMap {
    fn new() -> Self {
        let mut hash_table: Vec<Bucket> = Vec::new();
        let key_space = 2069;
        for _i in 0..key_space {
            hash_table.push(Bucket::default())
        }
        Self {
            key_space,
            hash_table,
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let hash_key = key % self.key_space;
        if let Some(bucket) = self.hash_table.get_mut(hash_key as usize) {
            (*bucket).update(key, value);
        }
    }

    fn get(&self, key: i32) -> i32 {
        let hash_key = key % self.key_space;
        if let Some(bucket) = self.hash_table.get(hash_key as usize) {
            return (*bucket).get(key);
        }
        return -1;
    }

    fn remove(&mut self, key: i32) {
        let hash_key = key % self.key_space;
        if let Some(bucket) = self.hash_table.get_mut(hash_key as usize) {
            (*bucket).remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::design_hash_map::MyHashMap;

    use super::Bucket;

    #[test]
    fn bucket_test() {
        let mut bucket = Bucket::default();
        bucket.update(1, 2);
        bucket.update(10, 27);
        assert_eq!(bucket.get(1), 2);
        assert_eq!(bucket.get(10), 27);
        bucket.remove(10);
        assert_eq!(bucket.get(10), -1);
    }

    #[test]
    fn my_hash_map_test() {
        let mut my_hash_map = MyHashMap::new();
        my_hash_map.put(1, 100);
        my_hash_map.put(2, 200);
        my_hash_map.put(3, 300);
        assert_eq!(my_hash_map.get(1), 100);
        my_hash_map.remove(3);
        assert_eq!(my_hash_map.get(3), -1);
    }
}
