// Answer 0

#[test]
fn test_new_with_non_empty_vector() {
    struct Bucket<T> {
        value: T,
    }

    struct MyCollection<T> {
        iter: std::vec::IntoIter<Bucket<T>>,
    }

    impl<T> MyCollection<T> {
        pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let buckets = vec![Bucket { value: 1 }, Bucket { value: 2 }];
    let collection = MyCollection::new(buckets);
    assert_eq!(collection.iter.len(), 2);
}

#[test]
fn test_new_with_empty_vector() {
    struct Bucket<T> {
        value: T,
    }

    struct MyCollection<T> {
        iter: std::vec::IntoIter<Bucket<T>>,
    }

    impl<T> MyCollection<T> {
        pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let buckets: Vec<Bucket<i32>> = Vec::new();
    let collection = MyCollection::new(buckets);
    assert!(collection.iter.len() == 0);
}

#[test]
#[should_panic]
fn test_new_with_panic_condition() {
    struct Bucket<T> {
        value: T,
    }

    struct MyCollection<T> {
        iter: std::vec::IntoIter<Bucket<T>>,
    }

    impl<T> MyCollection<T> {
        pub(super) fn new(entries: Vec<Bucket<T>>) -> Self {
            Self {
                iter: entries.into_iter(),
            }
        }
    }

    let buckets: Vec<Bucket<i32>> = Vec::new();
    let _collection = MyCollection::new(buckets);
    let _ = _collection.iter.next().expect("Iterator should not panic on empty!"); // This will panic
}

