// Answer 0

#[cfg(test)]
mod tests {
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
            Map {
                entries: Vec::new(),
                index: 0,
            }
        }
        
        fn with_entries(entries: Vec<Entry<K, V>>) -> Self {
            Map {
                entries,
                index: 0,
            }
        }
        
        fn set_index(&mut self, index: usize) {
            self.index = index;
        }

        fn get_key_value(&self) -> (&K, &V) {
            self.entries[self.index].refs()
        }
        
        fn index(&self) -> usize {
            self.index
        }
    }

    impl<K, V> Entry<K, V> {
        fn refs(&self) -> (&K, &V) {
            (&self.key, &self.value)
        }
    }

    #[test]
    fn test_get_key_value_valid_entry() {
        let entries = vec![
            Entry { key: 1, value: "a" },
            Entry { key: 2, value: "b" },
        ];
        let map = Map::with_entries(entries);
        let (key, value) = map.get_key_value();
        assert_eq!(*key, 1);
        assert_eq!(*value, "a");
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_get_key_value_panic_out_of_bounds() {
        let map = Map::new();
        map.get_key_value();
    }

    #[test]
    fn test_get_key_value_change_index() {
        let entries = vec![
            Entry { key: 1, value: "a" },
            Entry { key: 2, value: "b" },
        ];
        let mut map = Map::with_entries(entries);
        map.set_index(1);
        let (key, value) = map.get_key_value();
        assert_eq!(*key, 2);
        assert_eq!(*value, "b");
    }
}

