use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Default)]
pub struct InsertionOrderMap<K, V> {
    data: Vec<(K, V)>,
    indices: HashMap<K, usize>,
}

impl<K: Eq + Ord + Clone + std::hash::Hash, V> InsertionOrderMap<K, V> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            indices: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(&pos) = self.indices.get(&key) {
            self.data[pos].1 = value;
        } else {
            let pos = self.data.len();
            self.data.push((key.clone(), value));
            self.indices.insert(key, pos);
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.indices.get(key).map(|&pos| &self.data[pos].1)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.data.iter().map(|(k, v)| (k, v))
    }

    pub fn into_iter(self) -> impl Iterator<Item = (K, V)> {
        self.data.into_iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.data.iter().map(|(k, _)| k)
    }

    pub fn update_value_for_key<F>(&mut self, key: &K, update_fn: F)
    where
        F: FnOnce(&mut V),
    {
        if let Some(&pos) = self.indices.get(key) {
            update_fn(&mut self.data[pos].1);
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.data.shuffle(&mut rng);
        
        for (i, (k, _)) in self.data.iter().enumerate() {
            self.indices.insert(k.clone(), i);
        }
    }

    pub fn sort_by_key(&mut self) {
        self.data.sort_by(|a, b| a.0.cmp(&b.0));
        
        for (i, (k, _)) in self.data.iter().enumerate() {
            self.indices.insert(k.clone(), i);
        }
    }
}