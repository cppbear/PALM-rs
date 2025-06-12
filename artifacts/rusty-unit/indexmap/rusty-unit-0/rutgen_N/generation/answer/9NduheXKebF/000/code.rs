// Answer 0

#[test]
fn test_retain2_keeps_elements() {
    struct TestStruct {
        value: i32,
    }

    let mut index_map = indexmap::IndexMap::new();
    index_map.insert("a", TestStruct { value: 1 });
    index_map.insert("b", TestStruct { value: 2 });
    index_map.insert("c", TestStruct { value: 3 });

    index_map.retain2(|item| {
        item.value > 1
    });

    assert_eq!(index_map.len(), 2); // only "b" and "c" should remain
}

#[test]
fn test_retain2_removes_all_elements() {
    struct TestStruct {
        value: i32,
    }

    let mut index_map = indexmap::IndexMap::new();
    index_map.insert("a", TestStruct { value: 1 });
    index_map.insert("b", TestStruct { value: 2 });
    index_map.insert("c", TestStruct { value: 3 });

    index_map.retain2(|item| {
        item.value > 5
    });

    assert_eq!(index_map.len(), 0); // all elements should be removed
}

#[test]
fn test_retain2_keeps_no_elements_when_empty() {
    struct TestStruct {
        value: i32,
    }

    let mut index_map: indexmap::IndexMap<&str, TestStruct> = indexmap::IndexMap::new();

    index_map.retain2(|item| {
        item.value > 1
    });

    assert_eq!(index_map.len(), 0); // remains empty
}

