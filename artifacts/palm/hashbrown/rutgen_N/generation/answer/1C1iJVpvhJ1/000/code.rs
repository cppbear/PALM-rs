// Answer 0

#[test]
fn test_iter_empty() {
    use hashbrown::HashMap;
    
    let map: HashMap<i32, i32> = HashMap::new();
    let mut iter = map.iter();
    
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_item() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 10);
    let mut iter = map.iter();
    
    assert_eq!(iter.next(), Some((&1, &10)));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_multiple_items() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    let mut iter = map.iter();

    assert_eq!(iter.next(), Some((&1, &10)));
    assert_eq!(iter.next(), Some((&2, &20)));
    assert_eq!(iter.next(), Some((&3, &30)));
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_after_consuming() {
    use hashbrown::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    let mut iter = map.iter();
    
    assert_eq!(iter.next(), Some((&1, &10)));
    // Consume the first item
    let _ = iter.next();
    // Now check the remaining items
    assert_eq!(iter.next(), Some((&2, &20)));
    assert!(iter.next().is_none());
}

