// Answer 0

#[test]
fn test_map_iter_new_non_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MapIter<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries = vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];

    let map_iter = MapIter::new(&entries);
    assert_eq!(map_iter.iter.len(), entries.len());
}

#[test]
fn test_map_iter_new_empty() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MapIter<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries: Vec<Bucket<i32, &str>> = vec![];
    
    let map_iter = MapIter::new(&entries);
    assert_eq!(map_iter.iter.len(), entries.len());
}

