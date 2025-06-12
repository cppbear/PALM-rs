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
    let my_struct = MyStruct::new(&entries);
    assert!(my_struct.iter.len() == 0);
}

#[test]
fn test_new_with_one_entry() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyStruct<'a, K, V> {
        iter: std::slice::Iter<'a, Bucket<K, V>>,
    }

    let entries: Vec<Bucket<i32, i32>> = vec![Bucket { key: 1, value: 2 }];
    let my_struct = MyStruct::new(&entries);
    assert_eq!(my_struct.iter.len(), 1);
    assert_eq!(my_struct.iter.next().map(|b| b.key), Some(1));
    assert_eq!(my_struct.iter.next().map(|b| b.value), Some(2));
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

    let entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { key: 1, value: 2 },
        Bucket { key: 3, value: 4 },
        Bucket { key: 5, value: 6 },
    ];
    let my_struct = MyStruct::new(&entries);
    
    let values: Vec<_> = my_struct.iter.clone().map(|b| (b.key, b.value)).collect();
    assert_eq!(values, vec![(1, 2), (3, 4), (5, 6)]);
}

