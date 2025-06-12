// Answer 0

#[test]
fn test_new_creates_iterator() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyStruct<K, V> {
        iter: std::vec::IntoIter<Bucket<K, V>>,
    }

    impl<K, V> MyStruct<K, V> {
        pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let buckets = vec![
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];
    
    let my_struct = MyStruct::new(buckets);
    
    assert_eq!(my_struct.iter.next().unwrap().key, 1);
    assert_eq!(my_struct.iter.next().unwrap().value, "a");
}

#[test]
fn test_new_with_empty_vector() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct MyStruct<K, V> {
        iter: std::vec::IntoIter<Bucket<K, V>>,
    }

    impl<K, V> MyStruct<K, V> {
        pub(super) fn new(entries: Vec<Bucket<K, V>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let buckets: Vec<Bucket<i32, &str>> = vec![];
    
    let my_struct = MyStruct::new(buckets);
    
    assert!(my_struct.iter.next().is_none());
}

