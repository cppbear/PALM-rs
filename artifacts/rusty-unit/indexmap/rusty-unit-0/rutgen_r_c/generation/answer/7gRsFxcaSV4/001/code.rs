// Answer 0

#[test]
fn test_first_entry_empty_map() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>,
    }
    
    impl<K, V> TestMap<K, V> {
        fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {
            self.get_index_entry(0)
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {
            if index >= self.core.entries.as_entries().len() {
                return None;
            }
            Some(IndexedEntry { 
                map: RefMut::map(&mut self.core, |core| &mut core.entries), 
                index 
            })
        }
    }

    let mut map: TestMap<u32, String> = TestMap { core: IndexMapCore { indices: Vec::new(), entries: Vec::new() } };
    assert_eq!(map.first_entry(), None);
}

#[test]
fn test_first_entry_single_item() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {
            self.get_index_entry(0)
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {
            if index >= self.core.entries.as_entries().len() {
                return None;
            }
            Some(IndexedEntry { 
                map: RefMut::map(&mut self.core, |core| &mut core.entries), 
                index 
            })
        }
    }

    let mut map: TestMap<u32, String> = TestMap { core: IndexMapCore { indices: vec![0], entries: vec![("key".to_string(), "value".to_string())] } };
    if let Some(entry) = map.first_entry() {
        assert_eq!(entry.index, 0);
    } else {
        panic!("Expected an entry when map has one item");
    }
}

#[test]
fn test_first_entry_multiple_items() {
    struct TestMap<K, V> {
        core: IndexMapCore<K, V>,
    }

    impl<K, V> TestMap<K, V> {
        fn first_entry(&mut self) -> Option<IndexedEntry<'_, K, V>> {
            self.get_index_entry(0)
        }

        fn get_index_entry(&mut self, index: usize) -> Option<IndexedEntry<'_, K, V>> {
            if index >= self.core.entries.as_entries().len() {
                return None;
            }
            Some(IndexedEntry { 
                map: RefMut::map(&mut self.core, |core| &mut core.entries), 
                index 
            })
        }
    }

    let mut map: TestMap<u32, String> = TestMap { core: IndexMapCore { indices: vec![0, 1], entries: vec![("key1".to_string(), "value1".to_string()), ("key2".to_string(), "value2".to_string())] } };
    if let Some(entry) = map.first_entry() {
        assert_eq!(entry.index, 0);
    } else {
        panic!("Expected an entry when map has multiple items");
    }
}

