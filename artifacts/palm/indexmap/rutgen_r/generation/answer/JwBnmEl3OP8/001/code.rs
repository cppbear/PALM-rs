// Answer 0

#[test]
fn test_new_with_non_empty_slice() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IterWrapper<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }

    let mut buckets = [
        Bucket { key: 1, value: "a" },
        Bucket { key: 2, value: "b" },
    ];

    let iter_wrapper = IterWrapper::new(&mut buckets);
    let mut iter = iter_wrapper.iter;

    assert_eq!(iter.next().unwrap().key, 1);
    assert_eq!(iter.next().unwrap().key, 2);
}

#[test]
fn test_new_with_empty_slice() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IterWrapper<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }

    let mut buckets: [Bucket<i32, &str>; 0] = [];
    
    let iter_wrapper = IterWrapper::new(&mut buckets);
    let mut iter = iter_wrapper.iter;

    assert!(iter.next().is_none());
}

#[test]
#[should_panic]
fn test_new_with_uninitialized_slice() {
    struct Bucket<K, V> {
        key: K,
        value: V,
    }

    struct IterWrapper<'a, K, V> {
        iter: std::slice::IterMut<'a, Bucket<K, V>>,
    }

    let mut buckets: [&mut Bucket<i32, &str>; 0] = std::mem::MaybeUninit::uninit().assume_init();

    let _iter_wrapper = IterWrapper::new(&mut buckets);
}

