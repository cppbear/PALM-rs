// Answer 0

#[test]
fn test_contains_with_existing_value() {
    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.contains(&1), true);
}

#[test]
fn test_contains_with_non_existing_value() {
    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    assert_eq!(set.contains(&4), false);
}

#[test]
fn test_contains_with_borrowed_value() {
    let set: HashSet<String> = ["hello".to_string(), "world".to_string()].into_iter().collect();
    let borrowed: &str = "hello";
    assert_eq!(set.contains(&borrowed), true);
}

#[test]
fn test_contains_with_utf8_string() {
    let set: HashSet<&str> = ["foo", "bar"].into_iter().collect();
    assert_eq!(set.contains(&"foo"), true);
    assert_eq!(set.contains(&"baz"), false);
}

#[test]
fn test_contains_with_different_structs() {
    #[derive(Hash, Eq, PartialEq)]
    struct MyStruct {
        id: i32,
    }

    let set: HashSet<MyStruct> = vec![MyStruct { id: 1 }, MyStruct { id: 2 }].into_iter().collect();
    assert_eq!(set.contains(&MyStruct { id: 1 }), true);
    assert_eq!(set.contains(&MyStruct { id: 3 }), false);
}

#[test]
fn test_contains_with_empty_set() {
    let set: HashSet<i32> = HashSet::new();
    assert_eq!(set.contains(&1), false);
}

#[test]
fn test_contains_with_nil_value() {
    let set: HashSet<Option<i32>> = vec![Some(1), Some(2)].into_iter().collect();
    assert_eq!(set.contains(&Some(1)), true);
    assert_eq!(set.contains(&None), false);
}

