// Answer 0

#[derive(Debug)]
struct MyMap<K, T> {
    data: std::collections::HashMap<K, T>,
}

impl<K: std::hash::Hash + Eq, T> MyMap<K, T> {
    fn new() -> Self {
        MyMap {
            data: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: K, value: T) {
        self.data.insert(key, value);
    }

    fn get2(&self, key: &K) -> Option<&T> {
        self.data.get(key)
    }

    fn index(&self, index: K) -> &T {
        match self.get2(&index) {
            Some(val) => val,
            None => panic!("no entry found for key {:?}", index),
        }
    }
}

#[test]
fn test_index_success() {
    let mut my_map = MyMap::new();
    let key = "example_key";
    let value = "example_value";
    my_map.insert(key.to_string(), value.to_string());

    let result = my_map.index(key.to_string());
    assert_eq!(result, &"example_value".to_string());
}

#[test]
#[should_panic(expected = "no entry found for key \"missing_key\"")]
fn test_index_panic() {
    let my_map = MyMap::new();
    my_map.index("missing_key".to_string());
}

