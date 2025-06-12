// Answer 0

#[test]
fn test_new_with_non_empty_entries() {
    struct Bucket<T>(T);

    struct MyIter<'a, T> {
        iter: std::slice::Iter<'a, Bucket<T>>,
    }

    impl<'a, T> MyIter<'a, T> {
        pub(super) fn new(entries: &'a [Bucket<T>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }
    }

    let entries = [Bucket(1), Bucket(2), Bucket(3)];
    let my_iter = MyIter::new(&entries);
    assert_eq!(my_iter.iter.len(), 3);
}

#[test]
fn test_new_with_empty_entries() {
    struct Bucket<T>(T);

    struct MyIter<'a, T> {
        iter: std::slice::Iter<'a, Bucket<T>>,
    }

    impl<'a, T> MyIter<'a, T> {
        pub(super) fn new(entries: &'a [Bucket<T>]) -> Self {
            Self {
                iter: entries.iter(),
            }
        }
    }

    let entries: &[Bucket<i32>] = &[];
    let my_iter = MyIter::new(entries);
    assert_eq!(my_iter.iter.len(), 0);
}

