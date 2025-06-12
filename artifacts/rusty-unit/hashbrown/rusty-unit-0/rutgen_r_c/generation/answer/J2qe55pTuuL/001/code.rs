// Answer 0

#[test]
fn test_iter_empty_set() {
    let set: HashSet<&str> = HashSet::new();
    let mut iter = set.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_element_set() {
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("a");
    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(&"a"));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_elements_set() {
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("a");
    set.insert("b");
    let mut iter = set.iter();
    let mut found_a = false;
    let mut found_b = false;

    while let Some(element) = iter.next() {
        match element {
            &"a" => found_a = true,
            &"b" => found_b = true,
            _ => panic!("Unexpected element: {}", element),
        }
    }
    
    assert!(found_a);
    assert!(found_b);
}

#[test]
fn test_iter_after_clear() {
    let mut set: HashSet<&str> = HashSet::new();
    set.insert("a");
    set.insert("b");
    set.clear();
    let mut iter = set.iter();
    assert!(iter.next().is_none());
}

