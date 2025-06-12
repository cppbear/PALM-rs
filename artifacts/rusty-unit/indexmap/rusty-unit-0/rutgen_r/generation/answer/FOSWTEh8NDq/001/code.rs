// Answer 0

#[test]
fn test_new_with_non_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }
    
    struct IterStruct<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }
    
    let mut buckets: [Bucket<i32, &str>; 3] = [
        Bucket { key: 1, value: "one" },
        Bucket { key: 2, value: "two" },
        Bucket { key: 3, value: "three" },
    ];

    let iter_struct = IterStruct {
        iter: buckets.iter_mut(),
    };

    let mut expected_keys: Vec<i32> = iter_struct.iter.map(|b| b.key).collect();
    assert_eq!(expected_keys, vec![1, 2, 3]);
}

#[test]
fn test_new_with_empty_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }
    
    struct IterStruct<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }

    let mut buckets: [Bucket<i32, &str>; 0] = [];
    
    let iter_struct = IterStruct {
        iter: buckets.iter_mut(),
    };
    
    let expected_count = iter_struct.iter.count();
    assert_eq!(expected_count, 0);
}

#[test]
#[should_panic]
fn test_new_with_uninitialized_entries() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }
    
    struct IterStruct<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }

    let mut buckets: [Bucket<i32, &str>; 3] = [
        Bucket { key: 1, value: "one" },
        Bucket { key: 2, value: "two" },
        unsafe { std::mem::MaybeUninit::uninit().assume_init() }, // Uninitialized bucket
    ];

    let _iter_struct = IterStruct {
        iter: buckets.iter_mut(),
    };
}

