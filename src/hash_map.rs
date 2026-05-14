use std::hash::{DefaultHasher, Hash, Hasher};

struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub struct HashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
    len: usize,
    capacity: usize,
}

fn hash_key<K: Hash>(key: &K) -> usize {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        let len = 0;

        // Default capacity for no division by 0 with modulo
        let capacity = 8;

        Self {
            buckets: (0..capacity).map(|_| Vec::new()).collect(),
            len,
            capacity,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let len = 0;

        Self {
            buckets: (0..capacity).map(|_| Vec::new()).collect(),
            len,
            capacity,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.get_index(&key);

        // Override in case of duplicate keys
        for entry in &mut self.buckets[index] {
            if entry.key == key {
                entry.value = value;
                return;
            }
        }

        // Push new entry into the index
        self.buckets[index].push(Entry::new(key, value));
        self.len += 1;

        if self.load_factor() > 0.7 {
            self.rehash();
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let index = self.get_index(&key);

        let len = self.buckets[index].len();

        for i in 0..len {
            if self.buckets[index][i].key == key {
                return Some(&self.buckets[index][i].value);
            }
        }

        return None;
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        let index = self.get_index(&key);
        let len = self.buckets[index].len();

        for idx in 0..len {
            if self.buckets[index][idx].key == key {
                self.len -= 1;
                return Some(self.buckets[index].remove(idx).value);
            }
        }
        None
    }

    pub fn contains_key(&self, key: K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn get_index(&self, key: &K) -> usize {
        let index = hash_key(&key) % self.capacity;
        index
    }

    fn load_factor(&self) -> f64 {
        self.len as f64 / self.capacity as f64
    }

    fn rehash(&mut self) {
        self.capacity *= 2;
        let mut new_buckets: Vec<Vec<Entry<K, V>>> =
            (0..self.capacity).map(|_| Vec::new()).collect();

        let old_buckets = std::mem::replace(&mut self.buckets, Vec::new());

        old_buckets.into_iter().flatten().for_each(|entry| {
            let index = self.get_index(&entry.key);
            new_buckets[index].push(entry);
        });

        self.buckets = new_buckets;
    }
}
