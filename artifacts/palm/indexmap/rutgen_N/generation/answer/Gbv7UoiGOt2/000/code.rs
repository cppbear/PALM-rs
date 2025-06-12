// Answer 0

#[derive(Debug)]
struct Entry<'a, K, V> {
    key: K,
    value: V,
    // Assume that there's a method to get mutable references
}

#[derive(Debug)]
struct Map<'a, K, V> {
    entries: Vec<Entry<'a, K, V>>,
}

impl<'a, K, V> Map<'a, K, V> {
    fn new() -> Self {
        Map { entries: Vec::new() }
    }

    fn insert(&mut self, key: K, value: V) {
        self.entries.push(Entry { key, value });
    }

    fn index(&self) -> usize {
        // Just for testing, we'll assume we want the first entry's index.
        0
    }
}

impl<'a, K, V> Entry<'a, K, V> {
    fn muts(&mut self) -> (&'a mut K, &'a mut V) {
        (&mut self.key, &mut self.value)
    }
}

impl<'a, K, V> Map<'a, K, V> {
    pub fn into_key_value_mut(&mut self) -> (&'a mut K, &'a mut V) {
        let index = self.index();
        self.entries[index].muts()
    }
}

#[test]
fn test_into_key_value_mut() {
    let mut map: Map<String, i32> = Map::new();
    map.insert("key1".to_string(), 42);

    let (key, value) = map.into_key_value_mut();

    assert_eq!(*key, "key1".to_string());
    assert_eq!(*value, 42);

    *value += 1;
    assert_eq!(*value, 43);
}

#[test]
fn test_into_key_value_mut_empty() {
    let mut map: Map<String, i32> = Map::new();

    // Trying to access key_value_mut on an empty map should panic.
    let result = std::panic::catch_unwind(|| {
        map.into_key_value_mut();
    });

    assert!(result.is_err());
}

