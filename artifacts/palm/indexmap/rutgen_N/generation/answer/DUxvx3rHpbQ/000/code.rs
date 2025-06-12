// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

#[derive(Debug)]
struct SortedSet<K, V> {
    entries: Vec<Entry<K, V>>,
}

impl<K: Ord, V> SortedSet<K, V> {
    pub fn new() -> Self {
        SortedSet { entries: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.push(Entry { key, value });
        self.entries.sort_by(|a, b| a.key.cmp(&b.key));
    }

    pub fn binary_search_by<F>(&self, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&K) -> Ordering,
    {
        self.entries.binary_search_by(move |a| f(&a.key))
    }
}

#[test]
fn test_binary_search_by_found() {
    let mut set = SortedSet::new();
    set.insert(1, "a");
    set.insert(2, "b");
    set.insert(3, "c");

    let result = set.binary_search_by(|&key| key.cmp(&2));
    assert_eq!(result, Ok(1));
}

#[test]
fn test_binary_search_by_not_found() {
    let mut set = SortedSet::new();
    set.insert(1, "a");
    set.insert(3, "c");
    set.insert(4, "d");

    let result = set.binary_search_by(|&key| key.cmp(&2));
    assert_eq!(result, Err(1));
}

#[test]
fn test_binary_search_by_edge_case() {
    let mut set = SortedSet::new();
    set.insert(5, "e");
    set.insert(10, "j");

    let result = set.binary_search_by(|&key| key.cmp(&5));
    assert_eq!(result, Ok(0));

    let result = set.binary_search_by(|&key| key.cmp(&10));
    assert_eq!(result, Ok(1));

    let result = set.binary_search_by(|&key| key.cmp(&0));
    assert_eq!(result, Err(0));

    let result = set.binary_search_by(|&key| key.cmp(&15));
    assert_eq!(result, Err(2));
}

