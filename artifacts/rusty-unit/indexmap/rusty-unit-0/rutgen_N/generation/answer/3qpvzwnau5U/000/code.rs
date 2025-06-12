// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    struct TestStruct {
        map: indexmap::IndexMap<i32, ()>,
    }

    let mut test_struct = TestStruct {
        map: indexmap::IndexMap::new(),
    };

    test_struct.map.insert(1, ());
    test_struct.map.insert(2, ());
    test_struct.map.insert(3, ());

    assert_eq!(test_struct.shift_remove_index(1), Some(2)); // remove index 1 (value 2)
    assert_eq!(test_struct.map.len(), 2);
    assert!(test_struct.map.get(&1).is_some());
    assert!(test_struct.map.get(&3).is_some());
}

#[test]
fn test_shift_remove_index_empty() {
    struct TestStruct {
        map: indexmap::IndexMap<i32, ()>,
    }

    let mut test_struct = TestStruct {
        map: indexmap::IndexMap::new(),
    };

    assert_eq!(test_struct.shift_remove_index(0), None); // no elements to remove
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    struct TestStruct {
        map: indexmap::IndexMap<i32, ()>,
    }

    let mut test_struct = TestStruct {
        map: indexmap::IndexMap::new(),
    };

    test_struct.map.insert(1, ());
    test_struct.map.insert(2, ());

    assert_eq!(test_struct.shift_remove_index(2), None); // out of bounds
    assert_eq!(test_struct.map.len(), 2); // length should remain unchanged
}

