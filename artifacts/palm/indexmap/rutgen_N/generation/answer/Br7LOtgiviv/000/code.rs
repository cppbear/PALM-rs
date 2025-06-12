// Answer 0

#[test]
fn test_new_with_non_empty_entries() {
    struct Bucket<T> {
        value: T,
    }

    struct IteratorWrapper<T> {
        iter: std::vec::IntoIter<Bucket<T>>,
    }

    impl<T> IteratorWrapper<T> {
        pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let entries = vec![
        Bucket { value: 1 },
        Bucket { value: 2 },
        Bucket { value: 3 },
    ];

    let iterator = IteratorWrapper::new(entries);
    let mut collected_values: Vec<i32> = iterator.iter.map(|bucket| bucket.value).collect();
    
    assert_eq!(collected_values, vec![1, 2, 3]);
}

#[test]
fn test_new_with_empty_entries() {
    struct Bucket<T> {
        value: T,
    }

    struct IteratorWrapper<T> {
        iter: std::vec::IntoIter<Bucket<T>>,
    }

    impl<T> IteratorWrapper<T> {
        pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let entries: Vec<Bucket<i32>> = vec![];

    let iterator = IteratorWrapper::new(entries);
    let collected_values: Vec<i32> = iterator.iter.map(|bucket| bucket.value).collect();
    
    assert_eq!(collected_values, vec![]);
}

