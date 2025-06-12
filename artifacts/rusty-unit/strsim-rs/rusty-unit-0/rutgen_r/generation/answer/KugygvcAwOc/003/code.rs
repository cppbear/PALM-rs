// Answer 0

#[test]
fn test_lookup_with_unallocated_map() {
    struct TestStruct {
        mask: u32,
        map: Option<Vec<Item>>,
    }
    
    struct Item {
        key: u32,
        value: String,
    }

    let test_struct = TestStruct {
        mask: 0,
        map: None,
    };

    let panic_message = std::panic::catch_unwind(|| {
        test_struct.lookup(42);
    });

    assert!(panic_message.is_err());
}

#[test]
fn test_lookup_with_default_value() {
    struct TestStruct {
        mask: u32,
        map: Option<Vec<Item>>,
    }

    struct Item {
        key: u32,
        value: String,
    }

    let item = Item { key: 0, value: String::new() }; // Default value
    let test_struct = TestStruct {
        mask: 3,
        map: Some(vec![item.clone(), item.clone(), item.clone(), item.clone()]),
    };

    let result = test_struct.lookup(0);
    assert_eq!(result, 0);
}

#[test]
fn test_lookup_with_non_default_value() {
    struct TestStruct {
        mask: u32,
        map: Option<Vec<Item>>,
    }

    struct Item {
        key: u32,
        value: String,
    }

    let item1 = Item { key: 1, value: "value1".to_string() };
    let item2 = Item { key: 2, value: "value2".to_string() };
    
    let test_struct = TestStruct {
        mask: 3,
        map: Some(vec![item1.clone(), item2.clone(), item1.clone(), item2.clone()]),
    };

    let result = test_struct.lookup(1);
    assert_eq!(result, 0); // This will depend on the hash algorithm and setup.
}

#[test]
fn test_lookup_with_empty_map_entry() {
    struct TestStruct {
        mask: u32,
        map: Option<Vec<Item>>,
    }

    struct Item {
        key: u32,
        value: String,
    }

    let default_item = Item { key: 0, value: String::new() }; // Default value
    let test_struct = TestStruct {
        mask: 3,
        map: Some(vec![default_item.clone(), default_item.clone(), default_item.clone(), default_item.clone()]),
    };

    let result = test_struct.lookup(5); // 5 is arbitrary and should hit the default value case
    assert_eq!(result, 0); // Expecting it to return the first index
}

