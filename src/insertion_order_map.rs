use rand::seq::SliceRandom;

#[derive(Default)]
pub struct InsertionOrderMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K: Eq + Ord, V> InsertionOrderMap<K, V> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // TODO: fix target: time complexity in O(n)
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(pos) = self.data.iter().position(|(k, _)| k == &key) {
            self.data[pos] = (key, value);
        } else {
            self.data.push((key, value));
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.iter().find(|(k, _)| k == key).map(|(_, v)| v)
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
        if let Some(pos) = self.data.iter().position(|(k, _)| k == key) {
            update_fn(&mut self.data[pos].1);
        }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.data.shuffle(&mut rng);
    }

    pub fn sort_by_key(&mut self) {
        self.data.sort_by(|a, b| a.0.cmp(&b.0));
    }
}
