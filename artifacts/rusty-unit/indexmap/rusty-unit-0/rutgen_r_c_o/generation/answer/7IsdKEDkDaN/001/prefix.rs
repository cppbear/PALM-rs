// Answer 0

#[test]
fn test_sort_by_basic_ascending() {
    let mut map = IndexMap::new();
    map.insert(1, 100);
    map.insert(3, 300);
    map.insert(2, 200);
    map.sort_by(|k1, v1, k2, v2| k1.cmp(k2));
}

#[test]
fn test_sort_by_basic_descending() {
    let mut map = IndexMap::new();
    map.insert(1, 100);
    map.insert(3, 300);
    map.insert(2, 200);
    map.sort_by(|k1, v1, k2, v2| k2.cmp(k1));
}

#[test]
fn test_sort_by_same_keys() {
    let mut map = IndexMap::new();
    map.insert(2, 100);
    map.insert(2, 200);
    map.insert(2, 300);
    map.sort_by(|k1, v1, k2, v2| v1.cmp(v2));
}

#[test]
fn test_sort_by_complex_key_value() {
    let mut map = IndexMap::new();
    map.insert(2, 200);
    map.insert(1, 300);
    map.insert(3, 100);
    map.sort_by(|k1, v1, k2, v2| {
        if v1 == v2 {
            k1.cmp(k2)
        } else {
            v1.cmp(v2)
        }
    });
}

#[test]
fn test_sort_by_empty() {
    let mut map: IndexMap<u32, u32> = IndexMap::new();
    map.sort_by(|k1, v1, k2, v2| k1.cmp(k2));
}

#[test]
fn test_sort_by_large_input() {
    let mut map = IndexMap::with_capacity(10000);
    for i in (0..10000).rev() {
        map.insert(i, i);
    }
    map.sort_by(|k1, _, k2, _| k1.cmp(k2));
}

#[test]
fn test_sort_by_with_different_values() {
    let mut map = IndexMap::new();
    map.insert(5, 3);
    map.insert(1, 5);
    map.insert(3, 4);
    map.insert(2, 2);
    map.sort_by(|k1, v1, k2, v2| v1.cmp(v2));
}

