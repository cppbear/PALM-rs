// Answer 0

#[test]
fn test_iter_empty_map() {
    use hashbrown::HashMap;

    let map: HashMap<i32, String> = HashMap::new();
    let mut iter = map.iter();
    
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_single_item_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("one"));
    let mut iter = map.iter();
    
    let item = iter.next();
    assert!(item.is_some());
    assert_eq!(item.unwrap(), (&1, &String::from("one")));
}

#[test]
fn test_iter_multiple_items_map() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));
    map.insert(3, String::from("three"));
    let mut iter = map.iter();
    
    let mut collected: Vec<_> = iter.collect();
    collected.sort_by_key(|item| item.0); // Sort by key for comparison

    let expected = vec![(&1, &String::from("one")), 
                        (&2, &String::from("two")), 
                        (&3, &String::from("three"))];

    assert_eq!(collected, expected);
}

#[test]
fn test_iter_after_mutation() {
    use hashbrown::HashMap;

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("one"));
    map.insert(2, String::from("two"));

    let iter = map.iter();
    let mut collected: Vec<_> = iter.collect();

    assert_eq!(collected.len(), 2);
    
    map.insert(3, String::from("three"));

    let mut iter_after_mutation = map.iter();
    let new_collected: Vec<_> = iter_after_mutation.collect();
    
    assert_eq!(new_collected.len(), 3);
}

