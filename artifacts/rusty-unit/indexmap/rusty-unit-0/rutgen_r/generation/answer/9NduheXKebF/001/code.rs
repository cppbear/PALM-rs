// Answer 0

#[test]
fn test_retain2_with_keep_function_returning_true() {
    struct TestStruct {
        value: i32,
    }

    let mut map = indexmap::IndexMap::new();
    map.insert("a", TestStruct { value: 1 });
    map.insert("b", TestStruct { value: 2 });
    map.insert("c", TestStruct { value: 3 });

    let mut keep_condition = |item: &mut TestStruct| item.value > 0;

    map.retain2(&mut keep_condition);

    assert_eq!(map.len(), 3);
}

#[test]
fn test_retain2_with_keep_function_returning_false() {
    struct TestStruct {
        value: i32,
    }

    let mut map = indexmap::IndexMap::new();
    map.insert("a", TestStruct { value: 1 });
    map.insert("b", TestStruct { value: 2 });
    map.insert("c", TestStruct { value: -3 });

    let mut keep_condition = |item: &mut TestStruct| item.value > 0;

    map.retain2(&mut keep_condition);

    assert_eq!(map.len(), 2);
}

#[test]
fn test_retain2_with_empty_map() {
    struct TestStruct {
        value: i32,
    }

    let mut map: indexmap::IndexMap<&str, TestStruct> = indexmap::IndexMap::new();

    let mut keep_condition = |_: &mut TestStruct| false;

    map.retain2(&mut keep_condition);

    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain2_with_all_false_condition() {
    struct TestStruct {
        value: i32,
    }

    let mut map = indexmap::IndexMap::new();
    map.insert("a", TestStruct { value: 1 });
    map.insert("b", TestStruct { value: 2 });

    let mut keep_condition = |item: &mut TestStruct| item.value < 0;

    map.retain2(&mut keep_condition);

    assert_eq!(map.len(), 0);
}


