// Answer 0

#[test]
fn test_iter_mut() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    map.insert("key3", 3);

    let mut iter = map.iter_mut();
    
    assert_eq!(iter.next(), Some((&"key1", &mut 1)));
    assert_eq!(iter.next(), Some((&"key2", &mut 2)));
    assert_eq!(iter.next(), Some((&"key3", &mut 3)));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    let mut iter = map.iter_mut();
    
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut_after_modifications() {
    use indexmap::IndexMap;

    let mut map = IndexMap::new();
    map.insert("key1", 1);
    map.insert("key2", 2);
    
    {
        let mut iter = map.iter_mut();
        if let Some((_, v)) = iter.next() {
            *v += 1; // Modify value during iteration
        }
    }
    
    let mut iter = map.iter_mut();
    assert_eq!(iter.next(), Some((&"key1", &mut 2)));
    assert_eq!(iter.next(), Some((&"key2", &mut 2)));
    assert_eq!(iter.next(), None);
}

