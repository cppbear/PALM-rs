// Answer 0

#[test]
fn test_new_empty_entries() {
    struct Bucket<T> {
        data: T,
    }

    struct Iter<T> {
        iter: std::slice::Iter<'static, Bucket<T>>,
    }

    impl<T> Iter<T> {
        pub(super) fn new(entries: &'static [Bucket<T>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }
    }

    let entries: &'static [Bucket<i32>] = &[];
    let iter = Iter::new(entries);
    assert!(iter.iter.clone().count() == 0);
}

#[test]
fn test_new_non_empty_entries() {
    struct Bucket<T> {
        data: T,
    }

    struct Iter<T> {
        iter: std::slice::Iter<'static, Bucket<T>>,
    }

    impl<T> Iter<T> {
        pub(super) fn new(entries: &'static [Bucket<T>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }
    }

    let entries: &'static [Bucket<i32>] = &[Bucket { data: 1 }, Bucket { data: 2 }];
    let iter = Iter::new(entries);
    let count: usize = iter.iter.clone().count();
    assert!(count == 2);
}

