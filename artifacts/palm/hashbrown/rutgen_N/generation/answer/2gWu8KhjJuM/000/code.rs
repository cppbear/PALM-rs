// Answer 0

#[test]
fn test_index_found() {
    use hashbrown::HashMap;

    let map: HashMap<_, _> = [("a", "One"), ("b", "Two")].into();

    assert_eq!(map[&"a"], "One");
    assert_eq!(map[&"b"], "Two");
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_not_found() {
    use hashbrown::HashMap;

    let map: HashMap<_, _> = [("a", "One"), ("b", "Two")].into();

    let _ = map[&"c"]; // This should panic
}

