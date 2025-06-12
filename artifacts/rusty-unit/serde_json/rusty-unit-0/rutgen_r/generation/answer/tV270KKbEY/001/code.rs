// Answer 0

#[test]
fn test_into_values_empty_map() {
    struct TestMap {
        map: std::collections::HashMap<String, i32>,
    }

    impl TestMap {
        pub fn into_values(self) -> IntoValues {
            IntoValues {
                iter: self.map.into_values(),
            }
        }
    }

    struct IntoValues {
        iter: std::collections::hash_map::IntoValues<String, i32>,
    }

    let test_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    let values_iter = test_map.into_values().iter;

    assert_eq!(values_iter.count(), 0);
}

#[test]
fn test_into_values_single_element() {
    struct TestMap {
        map: std::collections::HashMap<String, i32>,
    }

    impl TestMap {
        pub fn into_values(self) -> IntoValues {
            IntoValues {
                iter: self.map.into_values(),
            }
        }
    }

    struct IntoValues {
        iter: std::collections::hash_map::IntoValues<String, i32>,
    }

    let mut test_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    test_map.map.insert("one".to_string(), 1);
    let values_iter = test_map.into_values().iter;

    let values: Vec<i32> = values_iter.collect();
    assert_eq!(values, vec![1]);
}

#[test]
fn test_into_values_multiple_elements() {
    struct TestMap {
        map: std::collections::HashMap<String, i32>,
    }

    impl TestMap {
        pub fn into_values(self) -> IntoValues {
            IntoValues {
                iter: self.map.into_values(),
            }
        }
    }

    struct IntoValues {
        iter: std::collections::hash_map::IntoValues<String, i32>,
    }

    let mut test_map = TestMap {
        map: std::collections::HashMap::new(),
    };
    test_map.map.insert("one".to_string(), 1);
    test_map.map.insert("two".to_string(), 2);
    test_map.map.insert("three".to_string(), 3);
    let values_iter = test_map.into_values().iter;

    let values: Vec<i32> = values_iter.collect();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_into_values_no_panic_on_large_map() {
    struct TestMap {
        map: std::collections::HashMap<String, i32>,
    }

    impl TestMap {
        pub fn into_values(self) -> IntoValues {
            IntoValues {
                iter: self.map.into_values(),
            }
        }
    }

    struct IntoValues {
        iter: std::collections::hash_map::IntoValues<String, i32>,
    }

    let mut test_map = TestMap {
        map: std::collections::HashMap::new(),
    };

    for i in 0..1000 {
        test_map.map.insert(i.to_string(), i);
    }

    let values_iter = test_map.into_values().iter;
    let values: Vec<i32> = values_iter.collect();
    assert_eq!(values.len(), 1000);
    assert_eq!(values[0], 0);
    assert_eq!(values[999], 999);
}

