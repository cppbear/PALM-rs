// Answer 0

#[test]
fn test_append_unchecked_empty_maps() {
    let mut map1: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let mut map2: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map1.append_unchecked(&mut map2);
    assert_eq!(map1.len(), 0);
}

#[test]
fn test_append_unchecked_non_empty_to_empty() {
    let mut map1: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let mut map2: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map2.insert(1, 10);
    map2.insert(2, 20);
    map1.append_unchecked(&mut map2);
    assert_eq!(map1.len(), 2);
    assert_eq!(map1.get(&1), Some(&10));
    assert_eq!(map1.get(&2), Some(&20));
    assert_eq!(map2.len(), 0);
}

#[test]
fn test_append_unchecked_empty_to_non_empty() {
    let mut map1: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let mut map2: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map1.insert(1, 10);
    map1.insert(2, 20);
    map1.append_unchecked(&mut map2);
    assert_eq!(map1.len(), 2);
    assert_eq!(map1.get(&1), Some(&10));
    assert_eq!(map1.get(&2), Some(&20));
}

#[test]
fn test_append_unchecked_non_empty_maps() {
    let mut map1: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let mut map2: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map1.insert(1, 10);
    map1.insert(2, 20);
    map2.insert(3, 30);
    map2.insert(4, 40);
    
    map1.append_unchecked(&mut map2);
    
    assert_eq!(map1.len(), 4);
    assert_eq!(map1.get(&1), Some(&10));
    assert_eq!(map1.get(&2), Some(&20));
    assert_eq!(map1.get(&3), Some(&30));
    assert_eq!(map1.get(&4), Some(&40));
    assert_eq!(map2.len(), 0);
}

#[test]
#[should_panic]
fn test_append_unchecked_invalid_operation() {
    let mut map1: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    let mut map2: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map1.append_unchecked(&mut map2);
    // Map2 is empty; this should not panic, but we can check the invariants.
}

