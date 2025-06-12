// Answer 0

#[test]
fn test_hashset_debug_empty() {
    let set: HashSet<i32> = HashSet { map: HashMap::default() };
    let output = format!("{:?}", set);
    assert_eq!(output, "{}");
}

#[test]
fn test_hashset_debug_single_element() {
    let mut set: HashSet<i32> = HashSet { map: HashMap::default() };
    set.map.insert(1, ());
    let output = format!("{:?}", set);
    assert_eq!(output, "{1}");
}

#[test]
fn test_hashset_debug_multiple_elements() {
    let mut set: HashSet<i32> = HashSet { map: HashMap::default() };
    set.map.insert(1, ());
    set.map.insert(2, ());
    set.map.insert(3, ());
    let output = format!("{:?}", set);
    assert!(output.contains("1"));
    assert!(output.contains("2"));
    assert!(output.contains("3"));
}

#[test]
fn test_hashset_debug_with_different_types() {
    let mut set: HashSet<&str> = HashSet { map: HashMap::default() };
    set.map.insert("foo", ());
    set.map.insert("bar", ());
    let output = format!("{:?}", set);
    assert!(output.contains("foo"));
    assert!(output.contains("bar"));
}

