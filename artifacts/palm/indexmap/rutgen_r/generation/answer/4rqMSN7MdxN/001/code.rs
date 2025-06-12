// Answer 0

#[test]
fn test_sort_unstable_by_empty_map() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.sort_unstable_by(|_k1, _v1, _k2, _v2| std::cmp::Ordering::Equal);
    assert!(map.is_empty());
}

#[test]
fn test_sort_unstable_by_single_element() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(1, 10);
    map.sort_unstable_by(|_k1, _v1, _k2, _v2| std::cmp::Ordering::Greater);
    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&1), Some(&10));
}

#[test]
fn test_sort_unstable_by_two_elements() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(2, 20);
    map.insert(1, 10);
    map.sort_unstable_by(|k1, v1, k2, v2| {
        if v1 < v2 {
            std::cmp::Ordering::Less
        } else if v1 > v2 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let expected = vec![(1, 10), (2, 20)];
    assert_eq!(map.iter().collect::<Vec<_>>(), expected);
}

#[test]
fn test_sort_unstable_by_multiple_elements() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(3, 30);
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(5, 15);
    
    map.sort_unstable_by(|k1, v1, k2, v2| {
        if v1 != v2 {
            v1.cmp(v2)
        } else {
            k1.cmp(k2)
        }
    });

    let expected = vec![(1, 10), (5, 15), (2, 20), (3, 30)];
    assert_eq!(map.iter().collect::<Vec<_>>(), expected);
}

#[test]
fn test_sort_unstable_by_same_value_different_keys() {
    let mut map: indexmap::IndexMap<i32, i32> = indexmap::IndexMap::new();
    map.insert(3, 10);
    map.insert(1, 10);
    map.insert(2, 10);
    
    map.sort_unstable_by(|k1, v1, k2, v2| v1.cmp(v2));

    let expected = vec![(3, 10), (1, 10), (2, 10)];
    assert_eq!(map.iter().collect::<Vec<_>>(), expected);
}

