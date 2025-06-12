// Answer 0

#[test]
fn test_sort_by_keys() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    let mut map = IndexMap::new();
    map.insert("b", 2);
    map.insert("a", 1);
    map.insert("c", 3);

    map.sort_by(|k1, v1, k2, v2| k1.cmp(k2));

    let expected: Vec<_> = vec![("a", 1), ("b", 2), ("c", 3)];
    assert_eq!(map.iter().collect::<Vec<_>>(), expected);
}

#[test]
fn test_sort_by_values() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    let mut map = IndexMap::new();
    map.insert("b", 3);
    map.insert("a", 1);
    map.insert("c", 2);

    map.sort_by(|k1, v1, k2, v2| v1.cmp(v2));

    let expected: Vec<_> = vec![("a", 1), ("c", 2), ("b", 3)];
    assert_eq!(map.iter().collect::<Vec<_>>(), expected);
}

#[test]
fn test_sort_by_keys_values_combination() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    let mut map = IndexMap::new();
    map.insert("b", 1);
    map.insert("a", 1);
    map.insert("c", 3);
    map.insert("d", 2);

    map.sort_by(|k1, v1, k2, v2| {
        let order = k1.cmp(k2);
        if order == Ordering::Equal {
            v1.cmp(v2)
        } else {
            order
        }
    });

    let expected: Vec<_> = vec![("a", 1), ("b", 1), ("d", 2), ("c", 3)];
    assert_eq!(map.iter().collect::<Vec<_>>(), expected);
}

#[test]
#[should_panic]
fn test_sort_empty_map() {
    use indexmap::IndexMap;

    let mut map: IndexMap<&str, i32> = IndexMap::new();
    map.sort_by(|k1, v1, k2, v2| k1.cmp(k2)); // should not panic
}

