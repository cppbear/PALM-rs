// Answer 0

#[test]
fn test_sorted_unstable_by_with_normal_data() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    let mut map = IndexMap::new();
    map.insert("b", 2);
    map.insert("a", 1);
    map.insert("c", 3);

    let sorted_entries: Vec<_> = map.sorted_unstable_by(|k1, v1, k2, v2| {
        if *v1 == *v2 {
            k1.cmp(k2)
        } else {
            v1.cmp(v2)
        }
    }).collect();

    assert_eq!(sorted_entries, vec![("a", 1), ("b", 2), ("c", 3)]);
}

#[test]
fn test_sorted_unstable_by_with_equal_values() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    let mut map = IndexMap::new();
    map.insert("x", 1);
    map.insert("y", 1);
    map.insert("z", 2);

    let sorted_entries: Vec<_> = map.sorted_unstable_by(|k1, v1, k2, v2| {
        if *v1 == *v2 {
            k1.cmp(k2)
        } else {
            v1.cmp(v2)
        }
    }).collect();

    assert_eq!(sorted_entries, vec![("x", 1), ("y", 1), ("z", 2)]);
}

#[test]
fn test_sorted_unstable_by_with_empty_map() {
    use indexmap::IndexMap;

    let map: IndexMap<i32, i32> = IndexMap::new();

    let sorted_entries: Vec<_> = map.sorted_unstable_by(|_, _, _, _| Ordering::Equal).collect();

    assert!(sorted_entries.is_empty());
}

#[test]
fn test_sorted_unstable_by_with_reverse_ordered_inserts() {
    use indexmap::IndexMap;
    use std::cmp::Ordering;

    let mut map = IndexMap::new();
    map.insert("c", 3);
    map.insert("b", 2);
    map.insert("a", 1);

    let sorted_entries: Vec<_> = map.sorted_unstable_by(|k1, v1, k2, v2| {
        if *v1 == *v2 {
            k1.cmp(k2)
        } else {
            v1.cmp(v2)
        }
    }).collect();

    assert_eq!(sorted_entries, vec![("a", 1), ("b", 2), ("c", 3)]);
}

