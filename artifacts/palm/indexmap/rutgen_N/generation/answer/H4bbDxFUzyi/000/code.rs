// Answer 0

#[test]
fn test_get_key_value_mut() {
    struct Entry<K, V> {
        key: K,
        value: V,
    }
    
    struct Map<K, V> {
        entries: Vec<Entry<K, V>>,
        index: usize,
    }
    
    impl<K, V> Map<K, V> {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                index: 0,
            }
        }

        fn index(&self) -> usize {
            self.index
        }

        fn add_entry(&mut self, key: K, value: V) {
            self.entries.push(Entry { key, value });
        }

        fn get_key_value_mut(&mut self) -> (&mut K, &mut V) {
            let index = self.index();
            let entry = &mut self.entries[index];
            (&mut entry.key, &mut entry.value)
        }
    }

    let mut map: Map<i32, String> = Map::new();
    map.add_entry(1, String::from("value1"));

    let (key, value) = map.get_key_value_mut();
    *key += 1; 
    value.push_str(" updated");

    assert_eq!(*key, 2);
    assert_eq!(*value, "value1 updated");
}

