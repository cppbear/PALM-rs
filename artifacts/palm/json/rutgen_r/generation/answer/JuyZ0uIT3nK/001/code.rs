// Answer 0

#[test]
fn test_values_mut_with_empty_map() {
    use std::collections::HashMap;
    
    struct MyMap {
        map: HashMap<String, i32>,
    }
    
    impl MyMap {
        pub fn values_mut(&mut self) -> ValuesMut {
            ValuesMut {
                iter: self.map.values_mut(),
            }
        }
    }
    
    struct ValuesMut<'a> {
        iter: std::collections::hash_map::ValuesMut<'a, String, i32>,
    }

    let mut my_map = MyMap { map: HashMap::new() };
    let mut values = my_map.values_mut();
    assert_eq!(values.iter.count(), 0);
}

#[test]
fn test_values_mut_with_one_element() {
    use std::collections::HashMap;
    
    struct MyMap {
        map: HashMap<String, i32>,
    }
    
    impl MyMap {
        pub fn values_mut(&mut self) -> ValuesMut {
            ValuesMut {
                iter: self.map.values_mut(),
            }
        }
    }
    
    struct ValuesMut<'a> {
        iter: std::collections::hash_map::ValuesMut<'a, String, i32>,
    }

    let mut my_map = MyMap { map: HashMap::new() };
    my_map.map.insert("a".to_string(), 1);
    let mut values = my_map.values_mut();
    *values.iter.next().unwrap() += 1; // Modifying the value
    assert_eq!(my_map.map["a"], 2);
}

#[test]
fn test_values_mut_with_multiple_elements() {
    use std::collections::HashMap;
    
    struct MyMap {
        map: HashMap<String, i32>,
    }
    
    impl MyMap {
        pub fn values_mut(&mut self) -> ValuesMut {
            ValuesMut {
                iter: self.map.values_mut(),
            }
        }
    }
    
    struct ValuesMut<'a> {
        iter: std::collections::hash_map::ValuesMut<'a, String, i32>,
    }

    let mut my_map = MyMap { map: HashMap::new() };
    my_map.map.insert("a".to_string(), 1);
    my_map.map.insert("b".to_string(), 2);
    my_map.map.insert("c".to_string(), 3);
    
    let mut values = my_map.values_mut();
    for value in values.iter {
        *value += 1; // Modifying the values
    }

    assert_eq!(my_map.map["a"], 2);
    assert_eq!(my_map.map["b"], 3);
    assert_eq!(my_map.map["c"], 4);
}

#[test]
fn test_values_mut_with_panic() {
    use std::collections::HashMap;
    
    struct MyMap {
        map: HashMap<String, i32>,
    }
    
    impl MyMap {
        pub fn values_mut(&mut self) -> ValuesMut {
            ValuesMut {
                iter: self.map.values_mut(),
            }
        }
    }
    
    struct ValuesMut<'a> {
        iter: std::collections::hash_map::ValuesMut<'a, String, i32>,
    }

    let mut my_map = MyMap { map: HashMap::new() };
    my_map.map.insert("key".to_string(), 42);
    
    let mut values = my_map.values_mut();
    
    // Consume all mutable references and check for panic when trying to use them again
    assert_eq!(values.iter.next().unwrap(), &42);
    my_map.map.clear(); // This invalidates the iterator
    let result = std::panic::catch_unwind(|| {
        values.iter.next();
    });
    
    assert!(result.is_err());
}

