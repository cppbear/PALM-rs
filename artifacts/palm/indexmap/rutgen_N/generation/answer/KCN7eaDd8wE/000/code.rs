// Answer 0

#[derive(Debug)]
struct MyMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> MyMap<K, V> {
    pub fn new() -> Self {
        MyMap { entries: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.push((key, value));
    }

    pub fn into_entries(self) -> Vec<(K, V)> {
        self.entries
    }

    pub fn into_values(self) -> Vec<V> {
        self.into_entries().into_iter().map(|(_, v)| v).collect()
    }
}

#[test]
fn test_into_values_non_empty_map() {
    let mut map = MyMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    let values = map.into_values();
    assert_eq!(values, vec!["value1", "value2"]);
}

#[test]
fn test_into_values_empty_map() {
    let map: MyMap<i32, i32> = MyMap::new();
    let values: Vec<i32> = map.into_values();
    assert_eq!(values, Vec::<i32>::new());
}

