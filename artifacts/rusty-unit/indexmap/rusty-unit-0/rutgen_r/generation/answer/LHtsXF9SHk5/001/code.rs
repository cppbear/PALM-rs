// Answer 0

#[test]
fn test_new_with_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyStruct<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries: Vec<Bucket<i32, i32>> = vec![];
    let result = MyStruct::new(&entries);
    assert!(result.iter.collect::<Vec<_>>().is_empty());
}

#[test]
fn test_new_with_single_entry() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyStruct<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries = vec![Bucket { key: 1, value: 100 }];
    let result = MyStruct::new(&entries);
    let collected: Vec<_> = result.iter.collect();
    assert_eq!(collected.len(), 1);
    assert_eq!(collected[0].key, 1);
    assert_eq!(collected[0].value, 100);
}

#[test]
fn test_new_with_multiple_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyStruct<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries = vec![
        Bucket { key: 1, value: 100 },
        Bucket { key: 2, value: 200 },
        Bucket { key: 3, value: 300 },
    ];
    let result = MyStruct::new(&entries);
    let collected: Vec<_> = result.iter.collect();
    assert_eq!(collected.len(), 3);
    assert_eq!(collected[0].key, 1);
    assert_eq!(collected[1].key, 2);
    assert_eq!(collected[2].key, 3);
}

#[test]
#[should_panic]
fn test_new_with_panic_on_borrowed_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyStruct<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries: Vec<Bucket<i32, i32>> = vec![Bucket { key: 1, value: 100 }];
    let _ = MyStruct::new(&entries);
    // Intentionally dropping `entries` after passing a reference to induce panic
    drop(entries);
}

