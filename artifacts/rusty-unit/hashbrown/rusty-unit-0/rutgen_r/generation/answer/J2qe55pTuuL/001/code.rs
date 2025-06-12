// Answer 0

#[test]
fn test_iter_empty_set() {
    use hashbrown::HashSet;

    let set: HashSet<&str> = HashSet::new();
    let mut iter = set.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_element() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.insert("a");
    
    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(&"a"));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_elements() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");
    
    let mut iter = set.iter();
    let elements: Vec<_> = iter.collect();
    let expected: Vec<&str> = vec!["a", "b", "c"];
    
    assert!(elements.len() == expected.len());
    for elem in expected {
        assert!(elements.contains(&elem));
    }
}

#[test]
fn test_iter_after_reinsert() {
    use hashbrown::HashSet;

    let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");

    let mut iter = set.iter();
    assert_eq!(iter.next(), Some(&"a"));
    
    set.insert("c");
    
    let mut iter_after = set.iter();
    let elements: Vec<_> = iter_after.collect();
    let expected: Vec<&str> = vec!["a", "b", "c"];
    
    assert!(elements.len() == expected.len());
    for elem in expected {
        assert!(elements.contains(&elem));
    }
}

