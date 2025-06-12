// Answer 0

#[test]
fn test_retain_with_some_elements() {
    use indexmap::IndexMap;
    
    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    
    map.retain(|_k, v| *v > 1);
    
    assert_eq!(map.len(), 2);
    assert!(map.contains_key("b"));
    assert!(map.contains_key("c"));
    assert!(!map.contains_key("a"));
}

#[test]
fn test_retain_with_no_elements() {
    use indexmap::IndexMap;
    
    let mut map: IndexMap<&str, i32> = IndexMap::new();
    
    map.retain(|_k, _v| true);
    
    assert_eq!(map.len(), 0);
}

#[test]
fn test_retain_with_all_elements_kept() {
    use indexmap::IndexMap;
    
    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    
    map.retain(|_k, _v| true);
    
    assert_eq!(map.len(), 2);
    assert!(map.contains_key("a"));
    assert!(map.contains_key("b"));
}

#[test]
fn test_retain_with_odd_conditions() {
    use indexmap::IndexMap;
    
    let mut map = IndexMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.insert("d", 4);
    
    map.retain(|k, v| {
        match k {
            &"a" => false,
            &"b" => *v % 2 == 0,
            &"c" => *v > 2,
            &"d" => true,
            _ => true,
        }
    });
    
    assert_eq!(map.len(), 3);
    assert!(!map.contains_key("a"));
    assert!(map.contains_key("b"));
    assert!(map.contains_key("c"));
    assert!(map.contains_key("d"));
}

