// Answer 0

#[test]
fn test_retain_with_all_elements_retained() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn retain<F>(&mut self, f: F)
        where
            F: FnMut(&String, &mut Value) -> bool,
        {
            self.map.retain(f);
        }
    }

    let mut my_map = MyMap {
        map: vec![
            (String::from("key1"), Value::from(1)),
            (String::from("key2"), Value::from(2)),
        ]
        .into_iter()
        .collect(),
    };

    my_map.retain(|_k, _v| true);

    assert_eq!(my_map.map.len(), 2);
}

#[test]
fn test_retain_with_no_elements_retained() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn retain<F>(&mut self, f: F)
        where
            F: FnMut(&String, &mut Value) -> bool,
        {
            self.map.retain(f);
        }
    }

    let mut my_map = MyMap {
        map: vec![
            (String::from("key1"), Value::from(1)),
            (String::from("key2"), Value::from(2)),
        ]
        .into_iter()
        .collect(),
    };

    my_map.retain(|_k, _v| false);

    assert_eq!(my_map.map.len(), 0);
}

#[test]
fn test_retain_with_some_elements_retained() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn retain<F>(&mut self, f: F)
        where
            F: FnMut(&String, &mut Value) -> bool,
        {
            self.map.retain(f);
        }
    }

    let mut my_map = MyMap {
        map: vec![
            (String::from("key1"), Value::from(1)),
            (String::from("key2"), Value::from(2)),
            (String::from("key3"), Value::from(3)),
        ]
        .into_iter()
        .collect(),
    };

    my_map.retain(|key, _v| key != "key2");

    assert_eq!(my_map.map.len(), 2);
    assert!(my_map.map.contains_key("key1"));
    assert!(my_map.map.contains_key("key3"));
    assert!(!my_map.map.contains_key("key2"));
}

#[test]
#[should_panic]
fn test_retain_with_panic_on_invalid_mutation() {
    use serde_json::Value;
    use std::collections::HashMap;

    struct MyMap {
        map: HashMap<String, Value>,
    }

    impl MyMap {
        pub fn retain<F>(&mut self, f: F)
        where
            F: FnMut(&String, &mut Value) -> bool,
        {
            self.map.retain(f);
        }
    }

    let mut my_map = MyMap {
        map: vec![
            (String::from("key1"), Value::from(1)),
        ]
        .into_iter()
        .collect(),
    };

    my_map.retain(|_key, v| {
        *v = Value::from(10); // This will mutate the value, which can trigger a panic in certain contexts
        false
    });
}

