// Answer 0

#[derive(Debug)]
struct HashValue(usize);

struct KeyValueMap<K, V> {
    map: std::collections::HashMap<HashValue, (K, V)>,
}

impl<K, V> KeyValueMap<K, V> {
    fn new() -> Self {
        KeyValueMap {
            map: std::collections::HashMap::new(),
        }
    }

    fn insert_unique(&mut self, hash: HashValue, key: K, value: V) -> &mut (K, V) {
        self.map.insert(hash, (key, value));
        self.map.get_mut(&hash).unwrap()
    }

    fn into_muts(&mut self) -> (&mut K, &mut V) {
        let (key, value) = self.map.values_mut().next().unwrap();
        (key, value)
    }
}

impl<K, V> KeyValueMap<K, V> {
    pub fn insert_hashed_nocheck(self, hash: u64, key: K, value: V) -> (&'static mut K, &'static mut V) {
        let hash = HashValue(hash as usize);
        self.insert_unique(hash, key, value).into_muts()
    }
}

#[test]
fn test_insert_hashed_nocheck() {
    let mut map: KeyValueMap<String, i32> = KeyValueMap::new();

    let (key, value) = map.insert_hashed_nocheck(12345, String::from("test_key"), 42);

    assert_eq!(*key, "test_key");
    assert_eq!(*value, 42);
}

#[test]
fn test_insert_hashed_nocheck_multiple() {
    let mut map: KeyValueMap<String, i32> = KeyValueMap::new();
    
    let (key1, value1) = map.insert_hashed_nocheck(12345, String::from("first_key"), 1);
    let (key2, value2) = map.insert_hashed_nocheck(67890, String::from("second_key"), 2);

    assert_eq!(*key1, "first_key");
    assert_eq!(*value1, 1);
    assert_eq!(*key2, "second_key");
    assert_eq!(*value2, 2);
}

