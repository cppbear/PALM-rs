// Answer 0

#[test]
fn test_contains_with_existing_value() {
    let mut set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let result = set.contains(&1);
}

#[test]
fn test_contains_with_non_existing_value() {
    let mut set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let result = set.contains(&4);
}

#[test]
fn test_contains_with_negative_value() {
    let mut set: HashSet<i32> = [-1, 0, 1].iter().cloned().collect();
    let result = set.contains(&-1);
}

#[test]
fn test_contains_with_zero_value() {
    let mut set: HashSet<i32> = [0, 1, 2].iter().cloned().collect();
    let result = set.contains(&0);
}

#[test]
fn test_contains_on_empty_set() {
    let set: HashSet<i32> = HashSet::new();
    let result = set.contains(&1);
}

#[test]
fn test_contains_with_string_key() {
    let mut set: HashSet<String> = HashSet::new();
    set.insert("Hello".to_string());
    let result = set.contains(&"Hello".to_string());
}

#[test]
fn test_contains_with_custom_struct() {
    #[derive(Hash, Eq, PartialEq)]
    struct Custom {
        id: i32,
    }

    let mut set: HashSet<Custom> = HashSet::new();
    set.insert(Custom { id: 1 });
    let result = set.contains(&Custom { id: 1 });
}

#[test]
fn test_contains_with_borrowed_value() {
    let mut set: HashSet<String> = ["foo", "bar"].iter().map(|&s| s.to_string()).collect();
    let b: &String = &"foo".to_string();
    let result = set.contains(b);
}

#[test]
fn test_contains_with_long_string() {
    let long_string = "a".repeat(1000);
    let mut set: HashSet<String> = HashSet::new();
    set.insert(long_string.clone());
    let result = set.contains(&long_string);
}

#[test]
#[should_panic]
fn test_contains_with_invalid_reference() {
    let mut set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let invalid_ref: &i32 = &4;
    let result = set.contains(invalid_ref);
}

