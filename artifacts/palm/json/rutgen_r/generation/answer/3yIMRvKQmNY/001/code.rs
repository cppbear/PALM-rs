// Answer 0

#[test]
fn test_values_with_empty_map() {
    use std::collections::HashMap;
    
    struct TestMap {
        map: HashMap<String, i32>,
    }

    impl TestMap {
        pub fn values(&self) -> Values {
            Values {
                iter: self.map.values(),
            }
        }
    }

    struct Values<'a> {
        iter: std::collections::hash_map::Values<'a, String, i32>,
    }

    let test_map = TestMap {
        map: HashMap::new(),
    };
    
    let values = test_map.values();
    let collected: Vec<&i32> = values.iter.collect();
    assert_eq!(collected.len(), 0);
}

#[test]
fn test_values_with_single_entry_map() {
    use std::collections::HashMap;

    struct TestMap {
        map: HashMap<String, i32>,
    }

    impl TestMap {
        pub fn values(&self) -> Values {
            Values {
                iter: self.map.values(),
            }
        }
    }

    struct Values<'a> {
        iter: std::collections::hash_map::Values<'a, String, i32>,
    }

    let mut test_map = TestMap {
        map: HashMap::new(),
    };
    test_map.map.insert("one".to_string(), 1);
    
    let values = test_map.values();
    let collected: Vec<&i32> = values.iter.collect();
    assert_eq!(collected, vec![&1]);
}

#[test]
fn test_values_with_multiple_entries_map() {
    use std::collections::HashMap;

    struct TestMap {
        map: HashMap<String, i32>,
    }

    impl TestMap {
        pub fn values(&self) -> Values {
            Values {
                iter: self.map.values(),
            }
        }
    }

    struct Values<'a> {
        iter: std::collections::hash_map::Values<'a, String, i32>,
    }

    let mut test_map = TestMap {
        map: HashMap::new(),
    };
    test_map.map.insert("one".to_string(), 1);
    test_map.map.insert("two".to_string(), 2);
    test_map.map.insert("three".to_string(), 3);
    
    let values = test_map.values();
    let collected: Vec<&i32> = values.iter.collect();
    assert_eq!(collected, vec![&1, &2, &3]);
}

