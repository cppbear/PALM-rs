// Answer 0

#[test]
fn test_remove() {
    use std::collections::HashMap;
    use crate::indexmap::{IndexMap, Entries};

    struct TestMap {
        map: IndexMap<i32, String>,
    }

    impl Entries for TestMap {
        type Entry = (i32, String);
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.map.into_iter().collect()
        }

        fn as_entries(&self) -> &[Self::Entry] {
            self.map.iter().collect::<Vec<_>>().as_slice()
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            self.map.iter_mut().collect::<Vec<_>>().as_mut_slice()
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.map.iter_mut().collect::<Vec<_>>());
        }
    }

    let mut test_map = TestMap { 
        map: IndexMap::new() 
    };
    
    test_map.map.insert(1, "value_1".to_string());
    test_map.map.insert(2, "value_2".to_string());

    let entry = test_map.map.get_key_value_mut(&1).unwrap();
    let key = entry.0;
    
    // Test remove functionality
    let value = entry.1.remove();

    assert_eq!(value, "value_1");
    assert!(test_map.map.get(&1).is_none());
}

#[test]
fn test_remove_empty_map() {
    use crate::indexmap::{IndexMap, Entries};

    struct TestMap {
        map: IndexMap<i32, String>,
    }

    impl Entries for TestMap {
        type Entry = (i32, String);
        
        fn into_entries(self) -> Vec<Self::Entry> {
            self.map.into_iter().collect()
        }

        fn as_entries(&self) -> &[Self::Entry] {
            self.map.iter().collect::<Vec<_>>().as_slice()
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            self.map.iter_mut().collect::<Vec<_>>().as_mut_slice()
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.map.iter_mut().collect::<Vec<_>>());
        }
    }

    let mut test_map = TestMap { 
        map: IndexMap::new() 
    };

    // Attempt to remove from an empty map
    let result = test_map.map.get_key_value_mut(&1);
    assert!(result.is_none());
}

