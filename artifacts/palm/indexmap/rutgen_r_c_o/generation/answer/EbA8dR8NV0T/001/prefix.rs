// Answer 0

#[test]
fn test_sorted_unstable_by_basic() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(3, 30);
    map.insert(1, 10);
    map.insert(2, 20);
    let _ = map.sorted_unstable_by(|k1, v1, k2, v2| {
        k1.cmp(k2).reverse()
    });
}

#[test]
fn test_sorted_unstable_by_empty() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    let _ = map.sorted_unstable_by(|_, _, _, _| Ordering::Equal);
}

#[test]
fn test_sorted_unstable_by_identical_keys() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(2, 20);
    map.insert(2, 30);
    map.insert(2, 10);
    let _ = map.sorted_unstable_by(|k1, v1, k2, v2| {
        v1.cmp(v2)
    });
}

#[test]
fn test_sorted_unstable_by_reverse_order() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(5, 50);
    map.insert(4, 40);
    map.insert(3, 30);
    let _ = map.sorted_unstable_by(|_, v1, _, v2| {
        v1.cmp(v2)
    });
}

#[test]
fn test_sorted_unstable_by_with_large_data() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    for i in 1..=1000 {
        map.insert(i, i * 10);
    }
    let _ = map.sorted_unstable_by(|k1, v1, k2, v2| {
        v1.cmp(v2).then(k1.cmp(k2))
    });
}

#[test]
fn test_sorted_unstable_by_duplicate_values() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap::new();
    map.insert(1, 20);
    map.insert(2, 20);
    map.insert(3, 10);
    let _ = map.sorted_unstable_by(|_, v1, _, v2| {
        v1.cmp(v2)
    });
}

