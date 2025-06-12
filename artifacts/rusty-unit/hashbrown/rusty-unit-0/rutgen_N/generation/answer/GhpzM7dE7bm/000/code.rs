// Answer 0

#[test]
fn test_try_insert_success() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    assert_eq!(map.try_insert(42, "value1").unwrap(), &"value1");
}

#[test]
fn test_try_insert_failure() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::OccupiedError;

    let mut map = HashMap::new();
    assert_eq!(map.try_insert(42, "value1").unwrap(), &"value1");

    match map.try_insert(42, "value2") {
        Err(OccupiedError { entry, value }) => {
            assert_eq!(entry.key(), &42);
            assert_eq!(entry.get(), &"value1");
            assert_eq!(value, "value2");
        }
        _ => panic!(),
    }
}

#[test]
fn test_try_insert_multiple_keys() {
    use hashbrown::HashMap;
    use hashbrown::hash_map::OccupiedError;

    let mut map = HashMap::new();
    assert_eq!(map.try_insert(1, "first").unwrap(), &"first");
    assert_eq!(map.try_insert(2, "second").unwrap(), &"second");

    match map.try_insert(1, "changed") {
        Err(OccupiedError { entry, value }) => {
            assert_eq!(entry.key(), &1);
            assert_eq!(entry.get(), &"first");
            assert_eq!(value, "changed");
        }
        _ => panic!(),
    }
}

